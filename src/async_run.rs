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

#[macro_export]
macro_rules! trace_async_fn {
    ($trace_id: expr, $async_fn:expr) => {
        match $async_fn.await {
            Ok(value) => {
                tracing::info!("trace_id: {} result: {:?}", $trace_id, value);
                Ok(value)
            }
            Err(err) => {
                tracing::error!("trace_id: {} error: {:?}", $trace_id, err);
                Err(err)
            }
        }
    };
}
