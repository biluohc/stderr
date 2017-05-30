[![Build status](https://travis-ci.org/biluohc/stderr.svg?branch=master)](https://github.com/biluohc/stderr)
[![Latest version](https://img.shields.io/crates/v/stderr.svg)](https://crates.io/crates/stderr)
[![All downloads](https://img.shields.io/crates/d/stderr.svg)](https://crates.io/crates/stderr)
[![Downloads of latest version](https://img.shields.io/crates/dv/stderr.svg)](https://crates.io/crates/stderr)
[![Documentation](https://docs.rs/stderr/badge.svg)](https://docs.rs/stderr)

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
# You only need `err,errln,errst,errstln` and don't want be polluted by other macros, you can
* use v0.3.0(The part has been stable since this version)

On toml

```toml
  [dependencies]
  stderr = "0.3.0"
```

## Or

* Only import you need by `macro_use`

```rust
  #[macro_use(err,errln,errst,errstln)]
  extern crate stderr;
```

# About `StaticMut` and `log::*`, please read [document](https://docs.rs/stderr)

## ChangLog
2017-0530 **0.8.0** `log::*, StaticMut`

2017-0424 **0.7.1** `loc!()` and `Loger::init(module_path!())`

2017-0211 **0.7.0** Supports multiple packages.

2017-0126 **0.6.1** Don't repeat initialization,fix #2.

2017-0126 **0.6.0** Refactoring and update API to `stderr::Loger; init!(),db!(),dbln!(),dbst!(),dbstln!()` for `LOG=module_path!()` and `--log/-log module_path!()`,add Documentation.

2017-0116 __0.5.0__ Add `stderr::loger::Loger;init(), debug!(),debugln!(),debugst!(),debugstln!()` for `LOG=debug` and `--log/-log debug`.
