#![crate_type = "dylib"]
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn jRustPrint(thing: *const c_char) {
  unsafe { 
    // Create a raw CStr from a raw ptr.
    let slice = CStr::from_ptr(thing);

    // Get a vector of bytes (slice) from the CStr and convert
    // it to a str. 
    let str = std::str::from_utf8(slice.to_bytes()).unwrap();

    // Create a String from str, send to function for printing.
    rustprint(str.to_string());
  }
}

fn rustprint(thing: String) {
  println!("hello from rust (using jna): {}", thing);
}
