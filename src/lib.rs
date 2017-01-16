#[macro_use]
pub mod loger;

// err!,errln!
#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        {
        use std::io::{self, Write};
        write!(&mut io::stderr(),$($arg)*).unwrap();
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
