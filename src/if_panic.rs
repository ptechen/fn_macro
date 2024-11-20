#[macro_export]
macro_rules! if_panic {
    ($func:expr) => {
        $func
    };

    ($func:expr, $err_val: expr) => {
        $func
    };
    // ($func:expr) => {
    //     match std::panic::catch_unwind(|| $func) {
    //         Ok(result) => result,
    //         Err(e) => {
    //             #[cfg(feature = "tracing")]
    //             tracing::error!("panic_info: {:?}", e.downcast_ref::<&str>());
    //             Default::default()
    //         }
    //     }
    // };
    //
    // ($func:expr, $err_val: expr) => {
    //     match std::panic::catch_unwind(|| $func) {
    //         Ok(result) => result,
    //         Err(e) => {
    //             #[cfg(feature = "tracing")]
    //             tracing::error!("panic_info: {:?}", e.downcast_ref::<&str>());
    //             $err_val
    //         }
    //     }
    // };
}
