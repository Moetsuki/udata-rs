#![allow(unused_imports)]
use crate::helpers::fmt_cef_string_utf16_t;

#[test]
fn test_cef_string() {
    //
    // Test conversion to and from rust types
    //
    let str0 = cef::CefString::from("Hello");

    assert_eq!(str0.to_string(), "Hello");

    //
    // Test conversion to and from cef types
    //
    let mut strz = "Hello".encode_utf16().collect::<Vec<u16>>();
    let str1 = cef::sys::_cef_string_utf16_t {
        str_: strz.as_mut_ptr(),
        length: 5,
        dtor: None,
    };

    assert_eq!(fmt_cef_string_utf16_t(&str1), "Hello");
}
