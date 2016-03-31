extern crate env_logger;
#[macro_use]
extern crate log;

use std::env;
use log::{LogRecord, LogLevelFilter};
use self::env_logger::LogBuilder;

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

  let format = |record: &LogRecord| {
      format!("{} - {}", record.level(), record.args())
  };

  let mut builder = LogBuilder::new();
  builder.format(format).filter(None, LogLevelFilter::Info);

  if env::var("MY_RUST_LIB_LOG").is_ok() {
      builder.parse(&env::var("MY_RUST_LIB_LOG").unwrap());
  }

  builder.init().unwrap();

  info!("In rust lib rustprint function, got '{}' as an argument", thing);

  println!("hello from rust (using jna): {}", thing);

}
