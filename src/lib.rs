
//!# [stderr](https://github.com/biluohc/stderr)
//!  A library that using macro to write to `io::stderr()` like `print!()/println!()`.
//!
//!## Usage
//!
//! On Cargo.toml:
//!
//! ```toml
//!  [dependencies]
//!  stderr = "0.7.0"
//! ```
//! or
//!
//! ```toml
//!  [dependencies]
//!  poolite = { git = "https://github.com/biluohc/stderr",branch = "master", version = "0.7.0" }
//! ```
//!
//!## About stderr
//!Usage as same as `print!/println!`.
//!
//!1. `err!`/`errln!`: Panics if writing to `io::stderr()` fails.
//!2. `errst!`/`errstln!`: Do nothing if writing to `io::stderr()` fails(silent->st).
//!
//!
//!## Example
//!
//!```Rust
//!#[macro_use]
//!extern crate stderr;
//!
//!fn main() {
//!  println!("err!/errln!/errst!/errstln!()@stderr !");
//!  let vec = vec![1, 2, 3, 4, 5];
//!  let s = std::env::args().nth(0).unwrap();
//!
//!  err!("err!(expr\\n)\n");
//!  err!("err!(String: vec![1, 2, 3, 4, 5])\n{}: {:?}", s, vec);
//!  errln!();
//!  errln!("errln!(expr)");
//!
//!  println!();
//!
//!  errst!("errst!(expr\\n)\n");
//!  errst!("errst!(String: vec![1, 2, 3, 4, 5])\n{}: {:?}", s, vec);
//!  errstln!();
//!  errstln!("errstln!(expr)");
//!
//!  // If you need to use `dbxxx!`,you must run `init!()` before use them on current process.
//!  // Otherwise you should ignore the following.
//!
//!  use stderr::Loger;  // `dbxxx!` belongs the module.
//!  init!();
//!  dbln!();
//!  dbln!("db!/dbln!()@Loger !");
//!  db!("{}\n", s);
//!  dbln!("{:?}", vec);
//!
//!  dbstln!();
//!  dbstln!("dbst!/dbstln!()@Loger !");
//!  dbst!("{}\n", s);
//!  dbstln!("{:?}", vec);
//!}
//!```
//!
//!## About stderr::Loger
//!`db!()`/`dbln!()`/`dbst!()`/`dbstln!()` print message while command line arguments conntains `-log` or `--log`(follows as [`module_path!()`](https://doc.rust-lang.org/std/macro.module_path.html),
//!if it has no followers(None) or followers by `''`,will print the message them occurred on current project(Use a stderr crate.)
//!
//!or environment variable 'LOG' as [`module_path!()`](https://doc.rust-lang.org/std/macro.module_path.html).
//!
//!if Value is None(`LOG=`) or (`LOG=''`),as same as above.
//!
//!Ps: fish can not set None.
//!### Example(if crate'name is 'app', a dependency's name is crate_lib):
//!
//!### Bash
//!
//!```bash
//!   env LOG=app  app_file_path
//!   env LOG=     app_file_path
//!   env LOG=''   app_file_path
//!   env LOG=app::mod1         app_file_path
//!   env LOG=app::mod1::mod2   app_file_path
//!   env LOG=* app_file_path   #use '*' to match all.
//!   env LOG=app,crate_lib   app_file_path     #print the message occurred on app or crate_lib
//!
//!   app_file_path --log app
//!   app_file_path --log
//!   app_file_path --log '' # use '' to avoid shell explain it.
//!
//!   app_file_path --log app::mod1
//!   app_file_path --log app::mod1::mod2
//!   app_file_path --log '*' # use '' to avoid shell explain it.
//!   app_file_path --log app,crate_lib
//!```
//!### Fish
//!```fish
//! set -x LOG app ; and app_file_path
//! set -x LOG '' ;and app_file_path
//! set -x LOG app::mod1 ;and app_file_path
//! set -x app::mod1::mod2 ;and app_file_path
//! set -x '*' ; and app_file_path
//! set -x 'app,crate_lib' ; and app_file_path
//!```
//!
//!If you neend to use `db!(),dbln!()` ,etc:
//!
//!You must use `init!()` before use them on the current process.
//!
#[macro_use]
mod log;
pub use log::Loger;

// err!,errln!
#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        {
        use std::io::{self, Write};
        write!(&mut io::stderr(),$($arg)*).unwrap();
        // Panics if writing to io::stdout() fails.        
        }
        };
}
#[macro_export]
macro_rules! errln {
       () => (err!("\n"));
       ($fmt:expr) => (err!(concat!($fmt, "\n")));
        ($fmt:expr, $($arg:tt)*) => (err!(concat!($fmt, "\n"), $($arg)*));
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
}
