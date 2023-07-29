use proc_macro2::Span;
use syn::punctuated::Punctuated;

#[cfg(feature = "alloc")]
pub struct CxxAbiArtifactInfo {
    pub path_components: ::alloc::vec::Vec<&'static str>,
    pub path_descendants: ::alloc::vec::Vec<&'static str>,
    pub cxx_include: &'static str,
    pub cxx_namespace: &'static str,
    pub cxx_name: &'static str,
    pub rust_name: &'static str,
    pub lifetimes: ::indexmap::IndexMap<&'static str, ::alloc::vec::Vec<&'static str>>,
    pub align: usize,
    pub size: usize,
    // pub is_cxx_trivial: bool,
    // pub is_cxx_copy_constructible: bool,
    // pub is_cxx_move_constructible: bool,
    // pub is_cxx_default_constructible: bool,
    // pub is_cxx_destructible: bool,
    // pub is_cxx_trivially_copyable: bool,
    // pub is_cxx_trivially_movable: bool,
    // pub is_cxx_trivially_destructible: bool,
    pub is_rust_cxx_extern_type_trivial: bool,
    pub is_rust_unpin: bool,
    pub is_rust_send: bool,
    pub is_rust_sync: bool,
    pub is_rust_copy: bool,
    pub is_rust_default: bool,
    pub is_rust_drop: bool,
    pub is_rust_copy_new: bool,
    pub is_rust_move_new: bool,
}

#[cfg(feature = "alloc")]
impl CxxAbiArtifactInfo {
    pub fn emit_file(&self) -> syn::File {
        let span = Span::call_site();
        let ident: &syn::Ident = &syn::Ident::new(self.rust_name, Span::call_site());
        let align = &proc_macro2::Literal::usize_unsuffixed(self.align);
        let size = &proc_macro2::Literal::usize_unsuffixed(self.size);
        let (generics_binder, generics) = {
            let all_static = false;
            &emit_generics(self, all_static)
        };
        let path_descendants = self.path_descendants.iter().map(|name| syn::Ident::new(name, span));
        let item_struct = emit_struct(self, align, size, ident, generics_binder, generics);
        let item_impl_cxx_extern_type = emit_impl_cxx_extern_type(self, ident, generics_binder, generics);
        let item_impl_drop = emit_impl_drop(self, ident, generics_binder, generics);
        let item_impl_debug = emit_impl_debug(self, ident, generics_binder, generics);
        let item_impl_default = emit_impl_default(self, ident, generics_binder, generics);
        let item_impl_moveit_copy_new = emit_impl_moveit_copy_new(self, ident, generics_binder, generics);
        let item_impl_moveit_move_new = emit_impl_moveit_move_new(self, ident, generics_binder, generics);
        let item_mod_cxx_bridge = emit_item_mod_cxx_bridge(self, ident, generics);
        let item_info_test_module = emit_info_test_module(self, ident, align, size);
        syn::parse_quote! {
            //! NOTE: This module is auto-generated and should not be edited.
            #(pub(crate) mod #path_descendants;)*
            #item_struct
            #item_impl_cxx_extern_type
            #item_impl_drop
            #item_impl_debug
            #item_impl_default
            #item_impl_moveit_copy_new
            #item_impl_moveit_move_new
            #item_mod_cxx_bridge
            #item_info_test_module
        }
    }

    #[cfg(feature = "std")]
    pub fn write_module_for_dir(path_components: &[&str], path_descendants: &[&str]) -> crate::BoxResult<()> {
        use quote::ToTokens;
        use rust_format::Formatter;
        let abi_out_dir_root = std::path::Path::new("src/abi");
        let abi_out_dir = abi_out_dir_root.join(std::path::PathBuf::from_iter(path_components));
        if let Some(parent) = abi_out_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let path = abi_out_dir.with_extension("rs");
        let file: syn::File = {
            let span = Span::call_site();
            let path_descendants = path_descendants
                .iter()
                .map(|descendant| syn::Ident::new(descendant, span));
            syn::parse_quote! {
                //! NOTE: This module is auto-generated and should not be edited.
                #(pub(crate) mod #path_descendants;)*
            }
        };
        let tokens = file.to_token_stream();
        let contents = rust_format::RustFmt::default().format_tokens(tokens)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    pub fn write_module_for_file(&self) -> crate::BoxResult<()> {
        use quote::ToTokens;
        use rust_format::Formatter;
        let abi_out_dir_root = std::path::Path::new("src/abi");
        let abi_out_dir = abi_out_dir_root.join(std::path::PathBuf::from_iter(&self.path_components));
        if let Some(parent) = abi_out_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let path = abi_out_dir.with_extension("rs");
        let file = self.emit_file();
        let tokens = file.to_token_stream();
        let contents = rust_format::RustFmt::default().format_tokens(tokens)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
}

#[cfg(feature = "alloc")]
fn emit_struct(
    info: &CxxAbiArtifactInfo,
    align: &proc_macro2::Literal,
    size: &proc_macro2::Literal,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> syn::ItemStruct {
    let attribute = emit_derive_attribute(info);
    let field_layout = field_layout(size);
    let field_neither_send_nor_sync = field_neither_send_nor_sync(info);
    let field_pinned = field_pinned(info);
    let field_lifetimes = field_lifetimes(generics);
    let fields = syn::FieldsNamed {
        brace_token: syn::token::Brace::default(),
        named: ::alloc::vec![
            Some(field_layout),
            field_neither_send_nor_sync,
            field_pinned,
            field_lifetimes,
        ]
        .into_iter()
        .filter_map(core::convert::identity)
        .collect::<Punctuated<syn::Field, syn::Token![,]>>(),
    };
    syn::parse_quote! {
        #attribute
        #[repr(C, align(#align))]
        pub struct #ident #generics_binder #fields
    }
}

#[cfg(feature = "alloc")]
fn emit_derive_attribute(info: &CxxAbiArtifactInfo) -> Option<syn::Attribute> {
    if info.is_rust_copy {
        Some(syn::parse_quote!(#[derive(Clone, Copy)]))
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn emit_field(name: &str, ty: syn::Type) -> syn::Field {
    syn::Field {
        attrs: ::alloc::vec![],
        vis: syn::Visibility::Inherited,
        mutability: syn::FieldMutability::None,
        ident: Some(syn::Ident::new(name, Span::call_site())),
        colon_token: Some(syn::Token![:](Span::call_site())),
        ty,
    }
}

#[cfg(feature = "alloc")]
fn emit_generics(info: &CxxAbiArtifactInfo, all_static: bool) -> (syn::Generics, syn::Generics) {
    let span = Span::call_site();
    let mut binder_params = Punctuated::<syn::GenericParam, syn::Token![,]>::new();
    let mut params = Punctuated::<syn::GenericParam, syn::Token![,]>::new();
    for (name, bounds) in &info.lifetimes {
        let name = if all_static { "static" } else { name };

        let lifetime = syn::Lifetime::new(&::alloc::format!("'{name}"), span);
        let lifetime_param = syn::LifetimeParam::new(lifetime);

        let mut lifetime_param_binder = lifetime_param.clone();
        for bound in bounds {
            let lifetime = syn::Lifetime::new(&::alloc::format!("'{bound}"), span);
            lifetime_param_binder.bounds.push_value(lifetime);
        }

        binder_params.push(syn::GenericParam::from(lifetime_param_binder));
        params.push(syn::GenericParam::from(lifetime_param))
    }
    let lt_token = if params.is_empty() {
        None
    } else {
        Some(syn::Token![<](span))
    };
    let gt_token = if params.is_empty() {
        None
    } else {
        Some(syn::Token![>](span))
    };
    let where_clause = None;
    (
        syn::Generics {
            lt_token,
            gt_token,
            params: binder_params,
            where_clause: where_clause.clone(),
        },
        syn::Generics {
            lt_token,
            gt_token,
            params,
            where_clause,
        },
    )
}

#[cfg(feature = "alloc")]
fn emit_impl_cxx_extern_type(
    info: &CxxAbiArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> syn::ItemImpl {
    let type_id = ::alloc::format!("{}::{}", info.cxx_namespace, info.cxx_name);
    let kind: syn::Type = if info.is_rust_cxx_extern_type_trivial {
        syn::parse_quote!(::cxx::kind::Trivial)
    } else {
        syn::parse_quote!(::cxx::kind::Opaque)
    };
    syn::parse_quote! {
        unsafe impl #generics_binder ::cxx::ExternType for #ident #generics {
            type Id = ::cxx::type_id!(#type_id);
            type Kind = #kind;
        }
    }
}

#[cfg(feature = "alloc")]
fn emit_impl_drop(
    info: &CxxAbiArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl> {
    if info.is_rust_drop {
        Some(syn::parse_quote! {
            impl #generics_binder ::core::ops::Drop for #ident #generics {
                #[cfg_attr(feature = "tracing", tracing::instrument)]
                #[inline]
                fn drop(&mut self) {
                    unsafe {
                        self::ffi::cxx_destruct(self);
                    }
                }
            }
        })
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn emit_impl_debug(
    info: &CxxAbiArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> syn::ItemImpl {
    let name = info.rust_name;
    syn::parse_quote! {
        #[cfg(feature = "debug")]
        impl #generics_binder ::core::fmt::Debug for #ident #generics {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(#name).finish()
            }
        }
    }
}

#[cfg(feature = "alloc")]
fn emit_impl_default(
    info: &CxxAbiArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl> {
    if info.is_rust_default {
        Some(syn::parse_quote! {
            impl #generics_binder #ident #generics {
                #[inline]
                pub(crate) fn default_new() -> impl ::cxx_memory::New<Output = #ident #generics> {
                    unsafe {
                        ::cxx_memory::new::by_raw(move |this| {
                            let this = this.get_unchecked_mut().as_mut_ptr();
                            self::ffi::cxx_default_new(this);
                        })
                    }
                }
            }
        })
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn emit_impl_moveit_copy_new(
    info: &CxxAbiArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl> {
    if info.is_rust_copy_new {
        Some(syn::parse_quote! {
            unsafe impl #generics_binder ::cxx_memory::CopyNew for #ident #generics {
                #[inline]
                unsafe fn copy_new(that: &Self, this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>) {
                    let this = this.get_unchecked_mut().as_mut_ptr();
                    self::ffi::cxx_copy_new(this, that)
                }
            }
        })
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn emit_impl_moveit_move_new(
    info: &CxxAbiArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl> {
    if info.is_rust_move_new {
        Some(syn::parse_quote! {
            unsafe impl #generics_binder ::cxx_memory::MoveNew for #ident #generics {
                #[inline]
                unsafe fn move_new(
                    that: ::core::pin::Pin<::cxx_memory::MoveRef<'_, Self>>,
                    this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>,
                ) {
                    let this = this.get_unchecked_mut().as_mut_ptr();
                    let that = &mut *::core::pin::Pin::into_inner_unchecked(that);
                    self::ffi::cxx_move_new(this, that)
                }
            }
        })
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn emit_info_test_module(
    info: &CxxAbiArtifactInfo,
    ident: &syn::Ident,
    align: &proc_macro2::Literal,
    size: &proc_macro2::Literal,
) -> syn::ItemMod {
    let (_, generics) = {
        let all_static = true;
        &emit_generics(info, all_static)
    };
    let static_assert_is_copy: Option<syn::ItemMacro> = if info.is_rust_copy {
        Some(syn::parse_quote!(
            ::static_assertions::assert_impl_all!(#ident #generics: ::core::marker::Copy);
        ))
    } else {
        None
    };
    let static_assert_is_unpin: Option<syn::ItemMacro> = if info.is_rust_unpin {
        Some(syn::parse_quote!(
            ::static_assertions::assert_impl_all!(#ident #generics: ::core::marker::Unpin);
        ))
    } else {
        None
    };
    syn::parse_quote! {
        #[cfg(test)]
        mod info {
            use super::*;
            mod test {
                use super::*;
                #[test]
                fn cxx_abi_align() {
                    ::core::assert_eq!(::core::mem::align_of::<#ident #generics>(), #align)
                }
                #[test]
                fn cxx_abi_size() {
                    ::core::assert_eq!(::core::mem::size_of::<#ident #generics>(), #size)
                }
                #static_assert_is_copy
                #static_assert_is_unpin
            }
        }
    }
}

#[cfg(feature = "alloc")]
fn emit_item_mod_cxx_bridge(info: &CxxAbiArtifactInfo, ident: &syn::Ident, generics: &syn::Generics) -> syn::ItemMod {
    let cxx_include = &info.cxx_include;
    let cxx_namespace = &info.cxx_namespace;
    let cxx_name = &info.cxx_name;
    let cxx_copy_new: Option<syn::ForeignItemFn> = if info.is_rust_copy_new {
        Some(syn::parse_quote! {
            unsafe fn cxx_copy_new #generics (This: *mut #ident #generics, that: &#ident #generics);
        })
    } else {
        None
    };
    let cxx_move_new: Option<syn::ForeignItemFn> = if info.is_rust_move_new {
        Some(syn::parse_quote! {
            unsafe fn cxx_move_new #generics (This: *mut #ident #generics, that: *mut #ident #generics);
        })
    } else {
        None
    };
    let cxx_default_new: Option<syn::ForeignItemFn> = if info.is_rust_default {
        Some(syn::parse_quote! {
            unsafe fn cxx_default_new #generics (This: *mut #ident #generics);
        })
    } else {
        None
    };
    let cxx_destruct: Option<syn::ForeignItemFn> = if info.is_rust_drop {
        Some(syn::parse_quote! {
            unsafe fn cxx_destruct #generics (This: *mut #ident #generics);
        })
    } else {
        None
    };
    syn::parse_quote! {
        #[cxx::bridge]
        pub(crate) mod ffi {
            #[namespace = #cxx_namespace]
            unsafe extern "C++" {
                include!(#cxx_include);

                #[cxx_name = #cxx_name]
                #[allow(unused)]
                type #ident #generics = super :: #ident #generics;
                #cxx_copy_new
                #cxx_move_new
                #cxx_default_new
                #cxx_destruct
            }
        }
    }
}

fn emit_refs_from_lifetimes(generics: &syn::Generics) -> Punctuated<syn::Type, syn::Token![,]> {
    generics
        .params
        .iter()
        .filter_map(|generic_param| emit_ref_type_from_lifetime(generic_param))
        .collect::<Punctuated<syn::Type, syn::Token![,]>>()
}

fn emit_ref_type_from_lifetime(generic_param: &syn::GenericParam) -> Option<syn::Type> {
    if let syn::GenericParam::Lifetime(lifetime_param) = generic_param {
        let lifetime = &lifetime_param.lifetime;
        Some(syn::parse_quote!(&#lifetime ()))
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn field_layout(size: &proc_macro2::Literal) -> syn::Field {
    let name = "_layout";
    let ty = syn::parse_quote!([u8; #size]);
    emit_field(name, ty)
}

#[cfg(feature = "alloc")]
fn field_neither_send_nor_sync(info: &CxxAbiArtifactInfo) -> Option<syn::Field> {
    let is_neither_send_nor_sync = !info.is_rust_send && !info.is_rust_sync;
    if is_neither_send_nor_sync {
        let name = "_neither_send_nor_sync";
        let ty = syn::parse_quote!(::core::marker::PhantomData<[*const u8; 0]>);
        Some(emit_field(name, ty))
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn field_pinned(info: &CxxAbiArtifactInfo) -> Option<syn::Field> {
    if info.is_rust_unpin {
        None
    } else {
        let name = "_pinned";
        let ty = syn::parse_quote!(::core::marker::PhantomPinned);
        Some(emit_field(name, ty))
    }
}

#[cfg(feature = "alloc")]
fn field_lifetimes(generics: &syn::Generics) -> Option<syn::Field> {
    let ref_types = emit_refs_from_lifetimes(generics);
    if ref_types.is_empty() {
        None
    } else {
        let name = "_lifetimes";
        let ty = syn::parse_quote!(::core::marker::PhantomData<(#ref_types,)>);
        Some(emit_field(name, ty))
    }
}
