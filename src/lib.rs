use std::io::{self, Write};
use std::error::Error;

// stderr!,stderrln!
pub fn stderr(msg: &str) {
    match io::stderr().write(msg.as_bytes()) {    
        Ok(_) => {}
        Err(e) => panic!("panic!: to stderr'{}' met '{}'", msg, e.description()),
        // Writing to standard error failed will panic this thread.
    };
}
#[macro_export]
macro_rules! stderr {
    ($($arg:tt)*) => {
        {let string=format!($($arg)*);
        stderr(&string);
        }
        };
}

#[macro_export]
macro_rules! stderrln {
    ($($arg:tt)*) => {
        {let string=format!($($arg)*)+"\n";
        stderr(&string);
        }
        };
}

// stderr_qt!,stderr_qtln!
pub fn stderr_qt(msg: &str) {
    match io::stderr().write(msg.as_bytes()) {    
        Ok(_) => {}
        Err(_) => {}  //Writing to standard error failed will do nothing.
    };
}
#[macro_export]
macro_rules! stderr_qt {
    ($($arg:tt)*) => {
        {let string=format!($($arg)*);
        stderr_qt(&string);
        }
        };
}
#[macro_export]
macro_rules! stderr_qtln {
    ($($arg:tt)*) => {
        {let string=format!($($arg)*)+"\n";
        stderr_qt(&string);
        }
        };
}
