/*!
# [stderr](https://github.com/biluohc/stderr)
  A library that using macro to write to `io::stderr()` like `print!()/println!()` for rust.

# Usage

 On Cargo.toml:

 ```toml
  [dependencies]
  stderr = "0.8.0"
 ```

# About stderr
Usage as same as `print!/println!`.

1. `err!`/`errln!`: Panics if writing to `io::stderr()` fails.
2. `errst!`/`errstln!`: Do nothing if writing to `io::stderr()` fails(silent->st).


# Example

```Rust
#[macro_use]
extern crate stderr;

fn main() {
  println!("err!/errln!/errst!/errstln!()@stderr !");
  let vec = vec![1, 2, 3, 4, 5];
  let s = std::env::args().nth(0).unwrap();

  err!("err!(expr\\n)\n");
  err!("err!(String: vec![1, 2, 3, 4, 5])\n{}: {:?}", s, vec);
  errln!();
  errln!("errln!(expr)");

  println!();

  errst!("errst!(expr\\n)\n");
  errst!("errst!(String: vec![1, 2, 3, 4, 5])\n{}: {:?}", s, vec);
  errstln!();
  errstln!("errstln!(expr)");
}
```
# If you only need `errxxx` and don't want be polluted by other macros, you can: 
* use v0.3.0(The part has been stable since this version)

On Cargo.toml:

```toml
  [dependencies]
  stderr = "0.3.0"
```

## Or

* Only import you need by `macro_use`

On Code:

```rustful
  #[macro_use(err,errln,errst,errstln)]
  extern crate stderr;
```

# About [stderr::StaticMut](struct.StaticMut.html)

[`StaticMut`](struct.StaticMut.html): Internal variability for [`Lazystatic`](https://crates.io/crates/lazy_static)

*/
#[macro_use]
extern crate lazy_static;
extern crate time;
/** `log module`

`dbxx!()`/`infoxx!()`/`warnxx!()`/`errorxx!()/fatalxx` print message while  environment variable 'LOG' or command line arguments conntains `-log` or `--log`

**Synntax: `LogLvl?/module_path,*`**

`Logger` will init by above value, stderr will print the message at `stderr()` if them's location and level pass `Logger::filter()`(Use a stderr crate.)

### Example(if crate'name is 'app', a dependency's name is map):

### Bash

```bash
   env LOG=/                # -> `all/app`
   env LOG="info/app"       # -> `info/app`
   env LOG="info/app,map"   # -> `info/app,map`
```
*/
///`    env LOG=info/*         # -> "info/*"`  ,`#` is all crate
/**
### Fish
```fish
 set -x LOG /                # -> `all/app`
 set -x LOG "info/app"       # -> `info/app`
 set -x LOG "info/app,map"   # -> `info/app,map`
 set -e LOG                  # remove environment variable
```
### Cli_Options_Argument

```sh
   ./app -log   /               # -> `all/app`
   ./app -log   info/app        # -> `info/app`
   ./app -log   info/app,map    # -> `info/app,map`
```
You must use `logger_init!()` before use them on the current process.
*/
pub mod log;
include!("static_mut.rs");

// err!,errln!
/// `print!()` for stderr
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
/// `println!()` for stderr
#[macro_export]
macro_rules! errln {
       () => (err!("\n"));
       ($fmt:expr) => (err!(concat!($fmt, "\n")));
        ($fmt:expr, $($arg:tt)*) => (err!(concat!($fmt, "\n"), $($arg)*));
}

// errst!,errstln!
/// `print!()` without `unwrap()` for stderr
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
/// `println!()` without `unwrap()` for stderr
#[macro_export]
macro_rules! errstln {
       () => (errst!("\n"));
       ($fmt:expr) => (errst!(concat!($fmt, "\n")));
        ($fmt:expr, $($arg:tt)*) => (errst!(concat!($fmt, "\n"), $($arg)*));
}
