use serde::Deserialize;

#[cfg(feature = "alloc")]
#[derive(Deserialize)]
pub struct CxxAbiEntry<'ctx> {
    cxx_include: &'ctx str,
    cxx_proxy_include: Option<&'ctx str>,
    cxx_namespace: &'ctx str,
    cxx_proxy_namespace: Option<&'ctx str>,
    cxx_name: Option<&'ctx str>,
    rust_name: &'ctx str,
    #[serde(default)]
    rust_lifetimes: ::indexmap::IndexMap<&'ctx str, ::alloc::vec::Vec<&'ctx str>>,
}

#[cfg(feature = "alloc")]
impl<'ctx> CxxAbiEntry<'ctx> {
    pub fn cxx_name(&self) -> &str {
        self.cxx_name.unwrap_or(self.rust_name)
    }

    pub(crate) fn emit_items_write_module_for_file<'a, 'b>(
        &self,
        path_components: impl Iterator<Item = &'a ::alloc::string::String>,
        path_descendants: impl Iterator<Item = &'b ::alloc::string::String>,
    ) -> ::alloc::vec::Vec<syn::ItemFn> {
        let cxx_include = self.cxx_include;
        let cxx_namespace = self.cxx_namespace;
        let cxx_name = self.cxx_name();
        let rust_name = self.rust_name;
        let lifetimes = {
            let mut exprs = ::alloc::vec::Vec::<syn::Expr>::new();
            for (lifetime, bounds) in &self.rust_lifetimes {
                exprs.push(syn::parse_quote!((#lifetime, vec![#(#bounds),*])));
            }
            exprs
        };
        ::alloc::vec![
            syn::parse_quote! {
                fn artifact_info() -> ::cxx_memory_abi::CxxAbiArtifactInfo {
                    let path_components = vec![#(#path_components),*];
                    let path_descendants = vec![#(#path_descendants),*];
                    let cxx_include = #cxx_include;
                    let cxx_namespace = #cxx_namespace;
                    let cxx_name = #cxx_name;
                    let rust_name = #rust_name;
                    let lifetimes = ::cxx_memory_abi::indexmap::IndexMap::from_iter([#(#lifetimes),*].into_iter());
                    let align = self::ffi::cxx_abi_align();
                    let size = self::ffi::cxx_abi_size();
                    let is_rust_cxx_extern_type_trivial = {
                        let cxx_is_trivially_movable = self::ffi::cxx_is_trivially_movable();
                        let rust_should_impl_cxx_extern_type_trivial = self::ffi::rust_should_impl_cxx_extern_type_trivial();
                        if cxx_is_trivially_movable == rust_should_impl_cxx_extern_type_trivial {
                            cxx_is_trivially_movable
                        } else {
                            rust_should_impl_cxx_extern_type_trivial
                        }
                    };
                    let is_rust_unpin = self::ffi::rust_should_impl_unpin();
                    let is_rust_send = self::ffi::rust_should_impl_send();
                    let is_rust_sync = self::ffi::rust_should_impl_sync();
                    let is_rust_copy = self::ffi::rust_should_impl_copy();
                    let is_rust_drop = self::ffi::rust_should_impl_drop();
                    let is_rust_default = self::ffi::rust_should_impl_default();
                    let is_rust_copy_new = self::ffi::rust_should_impl_cxx_memory_copy_new();
                    let is_rust_move_new = self::ffi::rust_should_impl_cxx_memory_move_new();
                    ::cxx_memory_abi::CxxAbiArtifactInfo {
                        path_components,
                        path_descendants,
                        cxx_include,
                        cxx_namespace,
                        cxx_name,
                        rust_name,
                        lifetimes,
                        align,
                        size,
                        is_rust_cxx_extern_type_trivial,
                        is_rust_unpin,
                        is_rust_send,
                        is_rust_sync,
                        is_rust_copy,
                        is_rust_default,
                        is_rust_drop,
                        is_rust_copy_new,
                        is_rust_move_new,
                    }
                }
            },
            syn::parse_quote! {
                pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
                    self::artifact_info().write_module_for_file()
                }
            },
        ]
    }

    pub(crate) fn emit_item_mod_cxx_bridge(&self) -> [syn::Item; 2] {
        let namespace: syn::Attribute = {
            let namespace = self.cxx_proxy_namespace.unwrap_or(self.cxx_namespace);
            syn::parse_quote!(#[namespace = #namespace])
        };
        let include = self.cxx_proxy_include.unwrap_or(self.cxx_include);
        [
            syn::parse_quote! {
                #[cxx::bridge]
                mod ffi {
                    #namespace
                    unsafe extern "C++" {
                        include!(#include);
                        fn cxx_abi_align() -> usize;
                        fn cxx_abi_size() -> usize;
                        fn cxx_is_copy_constructible() -> bool;
                        fn cxx_is_move_constructible() -> bool;
                        fn cxx_is_default_constructible() -> bool;
                        fn cxx_is_destructible() -> bool;
                        fn cxx_is_trivially_copyable() -> bool;
                        fn cxx_is_trivially_movable() -> bool;
                        fn cxx_is_trivially_destructible() -> bool;
                        fn rust_should_impl_cxx_extern_type_trivial() -> bool;
                        fn rust_should_impl_unpin() -> bool;
                        fn rust_should_impl_send() -> bool;
                        fn rust_should_impl_sync() -> bool;
                        fn rust_should_impl_drop() -> bool;
                        fn rust_should_impl_copy() -> bool;
                        fn rust_should_impl_default() -> bool;
                        fn rust_should_impl_cxx_memory_copy_new() -> bool;
                        fn rust_should_impl_cxx_memory_move_new() -> bool;
                    }
                }
            },
            syn::parse_quote! {
                pub use ffi::*;
            },
        ]
    }
}
