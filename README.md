# fn_macro

[![Version info](https://img.shields.io/crates/v/fn_macro.svg)](https://crates.io/crates/fn_macro)
[![Downloads](https://img.shields.io/crates/d/fn_macro.svg?style=flat-square)](https://crates.io/crates/fn_macro)
[![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/fn_macro)
[![dependency status](https://deps.rs/crate/fn_macro/0.1.3/status.svg)](https://deps.rs/crate/fn_macro)

# Example

```rust
    use fn_macro::prelude::*;
        
    #[test]
    fn test() {
        if_else!(true, 1, 2);
        hashmap!(1 => 2);
    }
```