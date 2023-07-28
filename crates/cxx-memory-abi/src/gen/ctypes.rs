// NOTE: cxx-build an empty bridge so that `cxx/include/**/*.hxx` is exported to dependencies
#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-memory-abi/cxx/include/cxx-memory-abi.hxx");

        #[cxx_name = "c_char"]
        type _c_char = crate::ffi::ctypes::c_char;
    }

    impl CxxVector<_c_char> {
    }
    impl UniquePtr<_c_char> {
    }
}
