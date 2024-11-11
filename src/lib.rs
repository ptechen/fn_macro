#[macro_use]
mod map;
#[macro_use]
mod async_run;

#[macro_use]
mod if_else;
#[macro_use]
mod if_unwrap_or_default;

#[macro_use]
mod if_ok_or_default;

#[macro_use]
mod if_panic;

#[macro_use]
mod set;

pub mod prelude {
    pub use crate::{if_else, if_unwrap_or_default, if_ok_or_default, if_panic,
                    async_fn, trace_async_fn, hashmap, hashset, btreemap, btreeset};
}