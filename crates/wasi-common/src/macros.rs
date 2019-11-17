macro_rules! hostcalls {
    ($(pub unsafe fn $name:ident($($arg:ident: $ty:ty,)*) -> $ret:ty;)*) => ($(
            #[wasi_common_cbindgen::wasi_common_cbindgen]
            pub unsafe fn $name($($arg: $ty,)*) -> $ret {
                let ret = crate::hostcalls_impl::$name($($arg,)*);
                log::trace!("{} --> {:?}", stringify!($name), ret);

                match ret {
                    Ok(()) => crate::wasi::__WASI_ESUCCESS,
                    Err(e) => e.as_wasi_errno(),
                }
            }
    )*)
}
