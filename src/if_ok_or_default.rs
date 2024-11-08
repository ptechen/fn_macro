#[macro_export]
macro_rules! if_ok_or_default {
    ($value:expr) => {
        if let Ok(value) = $value {
            value
        } else {
            tracing::error!("value is error");
            Default::default()
        }
    };
    ($value:expr, $default: expr) => {
        if let Ok(value) = $value {
            value
        } else {
            tracing::error!("value is error");
            $default
        }
    };
}