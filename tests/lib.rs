use fn_macro::prelude::*;
#[test]
fn test() {
    assert_eq!(1, if_else!(true, 1, 2));
    assert_eq!(1, if_unwrap_or_default!(Some(1)));
    assert_eq!(1, if_unwrap_or_default!(None, 1));
    assert_eq!(1, if_ok_or_default!(Ok::<i32, String>(1)));
    assert_eq!(1, if_ok_or_default!(Err(""), 1));
    let mut v:Option<i32>= None;
    assert_eq!(0, if_panic!(v.unwrap()));
    assert_eq!(1, if_panic!(v.unwrap(), 1));
    println!("{:?}", hashmap!(1 => 2));
    println!("{:?}", hashmap!(1 => 2, 2 => 3));
    println!("{:?}", btreeset!(1,2));
    println!("{:?}", hashset!(1,2));
}