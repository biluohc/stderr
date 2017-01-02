// err!,errln!
#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        {
        use std::io::{self, Write};
        // use std::error::Error;
        // let str=&format!($($arg)*);
        // match io::stderr().write(str.as_bytes()) {
        //     Ok(_) => {}
        //     Err(e) => panic!("panic!: to err! '{}' met '{}'", str, e.description()),
        //     // Panics if writing to io::stdout() fails.
        // };
        match write!(&mut io::stderr(),$($arg)* ) {
            Ok(..)=>{},
            Err(e)=>panic!(e),
        };
        }
        };
}
#[macro_export]
macro_rules! errln {
       () => (err!("\n"));
       ($fmt:expr) => (err!(concat!($fmt, "\n")));
        ($fmt:expr, $($arg:tt)*) => (err!(concat!($fmt, "\n"), $($arg)*));
        // Panics if writing to io::stdout() fails.
}

// errst!,errstln!
#[macro_export]
macro_rules! errst {
    ($($arg:tt)*) => {
        {
        use std::io::{self, Write};
        // let str=&format!($($arg)*);
        if let Ok(..) =  write!(&mut io::stderr(),$($arg)* ) {};
        // Do nothing if writing to io::stdout() fails(silent->st).
        }
        };
}
#[macro_export]
macro_rules! errstln {
       () => (errst!("\n"));
       ($fmt:expr) => (errst!(concat!($fmt, "\n")));
        ($fmt:expr, $($arg:tt)*) => (errst!(concat!($fmt, "\n"), $($arg)*));
        // Do nothing if writing to io::stdout() fails(silent->st).
}