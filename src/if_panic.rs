#[macro_export]
macro_rules! if_panic {
    ($func:expr) => {
        match std::panic::catch_unwind(|| $func) {
            Ok(result) => result,
            Err(e) => {
                panic!("{:#?}", e)
            }
        }
    };

    ($func:expr, $err_val: expr) => {
        match std::panic::catch_unwind(|| $func) {
            Ok(result) => result,
            Err(e) => {
                tracing::error!("panic_info: {:?}", e.downcast_ref::<&str>());
                $err_val
            }
        }
    };
}
