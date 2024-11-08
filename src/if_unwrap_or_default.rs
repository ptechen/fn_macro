#[macro_export]
macro_rules! if_unwrap_or_default {
    ($value:expr) => {
        if let Some(value) = $value {
            value
        } else {
            tracing::error!("value is unwrap");
            Default::default()
        }
    };
    ($value:expr, $default: expr) => {
        if let Some(value) = $value {
            value
        } else {
            tracing::error!("value is unwrap");
            $default
        }
    };
}