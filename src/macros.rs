#[macro_export]
macro_rules! let_cxx {
    (mut $name:ident $(: $ty:ty)? = &move *$expr:expr) => {
        $crate::let_cxx!(@move(mut) $name, $($ty)?, $expr)
    };
    ($name:ident $(: $ty:ty)? = &move *$expr:expr) => {
        $crate::let_cxx!(@move $name, $($ty)?, $expr)
    };
    (mut $name:ident $(: $ty:ty)? = &move $expr:expr) => {
        $crate::let_cxx!(@put(mut) $name, $($ty)?, $expr)
    };
    ($name:ident $(: $ty:ty)? = &move $expr:expr) => {
        $crate::let_cxx!(@put $name, $($ty)?, $expr)
    };
    (mut $name:ident $(: $ty:ty)? = $expr:expr) => {
        $crate::let_cxx!(@emplace(mut) $name, $($ty)?, $expr);
    };
    ($name:ident $(: $ty:ty)? = $expr:expr) => {
        $crate::let_cxx!(@emplace $name, $($ty)?, $expr);
    };
    (@move $(($mut:tt))? $name:ident, $($ty:ty)?, $expr:expr) => {
        let_cxx_slot!(#[dropping] slot);
        #[allow(unused_mut)]
        let $($mut)? $name $(: $ty)? = $crate::DerefMove::deref_move($expr, slot);
    };
    (@put $(($mut:tt))? $name:ident, $($ty:ty)?, $expr:expr) => {
        $crate::let_cxx_slot!(slot);
        let $($mut)? $name $(: $ty)? = slot.put($expr);
    };
    (@emplace $(($mut:tt))? $name:ident, $($ty:ty)?, $expr:expr) => {
        $crate::let_cxx_slot!(slot);
        let $($mut)? $name $(: $ty)? = slot.emplace($expr);
    };
}

#[macro_export]
macro_rules! cxx {
    (&move *$expr:expr) => {
        $crate::DerefMove::deref_move($expr, $crate::cxx_slot!(#[dropping]))
    };
    (&move $expr:expr) => {
        $crate::cxx_slot!().put($expr)
    };
    ($expr:expr) => {
        $crate::cxx_slot!().emplace($expr)
    };
}

#[macro_export]
macro_rules! cxx_slot {
    (#[dropping]) => {{
        let kind = $crate::SlotStorageKind::Drop;
        $crate::SlotStorage::new(kind).slot()
    }};
    () => {{
        let kind = $crate::SlotStorageKind::Keep;
        $crate::SlotStorage::new(kind).slot()
    }};
}

#[macro_export]
macro_rules! let_cxx_slot {
    (#[dropping] $($name:ident : $ty:ty),* $(,)?) => {
        $(
            let kind = $crate::SlotStorageKind::Drop;
            let mut storage = $crate::SlotStorage::<$ty>::new(kind);
            let $name = storage.slot();
        )*
    };
    (#[dropping] $($name:ident),* $(,)?) => {
        $(
            let kind = $crate::SlotStorageKind::Drop;
            let mut storage = $crate::SlotStorage::new(kind);
            let $name = storage.slot();
        )*
    };
    ($($name:ident : $ty:ty),* $(,)?) => {
        $(
            let kind = $crate::SlotStorageKind::Keep;
            let mut storage = $crate::SlotStorage::<$ty>::new(kind);
            let $name = storage.slot();
        )*
    };
    ($($name:ident),* $(,)?) => {
        $(
            let kind = $crate::SlotStorageKind::Keep;
            let mut storage = $crate::SlotStorage::new(kind);
            let $name = storage.slot();
        )*
    };
}

macro_rules! trivial_copy {
    ($($ty:ty $(where [$($targs:tt)*])?),* $(,)?) => {
        $(
            unsafe impl<$($($targs)*)?> $crate::new::CopyNew for $ty where Self: ::core::clone::Clone {
                unsafe fn copy_new(
                    this: &Self,
                    that: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>,
                ) {
                    let that = ::core::pin::Pin::into_inner_unchecked(that);
                    let data = this.clone();
                    that.write(data);
                }
            }
        )*
    }
}

macro_rules! trivial_move {
    ($($ty:ty $(where [$($targs:tt)*])?),* $(,)?) => {
        $(
            unsafe impl<$($($targs)*)?> $crate::new::MoveNew for $ty {
                unsafe fn move_new(
                    this: ::core::pin::Pin<$crate::move_ref::MoveRef<'_, Self>>,
                    that: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>,
                ) {
                    let this = ::core::pin::Pin::into_inner_unchecked(this);
                    let that = ::core::pin::Pin::into_inner_unchecked(that);
                    let data = $crate::move_ref::MoveRef::into_inner(this);
                    that.write(data);
                }
            }
        )*
    }
}
