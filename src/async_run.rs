#[macro_export]
macro_rules! async_fn {
    ($async_fn:expr) => {
        match $async_fn.await {
            Ok(value) => {
                tracing::info!("result: {:?}", value);
                Ok(value)
            }
            Err(err) => {
                tracing::error!("error: {:?}", err);
                Err(err)
            }
        }
    };
}
