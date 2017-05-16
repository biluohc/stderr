[![Build status](https://travis-ci.org/biluohc/stderr.svg?branch=master)](https://github.com/biluohc/stderr)
[![Latest version](https://img.shields.io/crates/v/stderr.svg)](https://crates.io/crates/stderr)
[![All downloads](https://img.shields.io/crates/d/stderr.svg)](https://crates.io/crates/stderr)
[![Downloads of latest version](https://img.shields.io/crates/dv/stderr.svg)](https://crates.io/crates/stderr)
[![Documentation](https://docs.rs/stderr/badge.svg)](https://docs.rs/stderr)

# A library that using macro to write to io::stderr() like print!()/println!() for Rust.

## Usage
Cargo.toml

```toml
[dependencies]
stderr = "0.8.0"
```
or
```toml
[dependencies]
stderr = { git = "https://github.com/biluohc/stderr", branch = "master", version = "0.8.0"}
```

## Documentation  
* Visit [https://docs.rs/stderr/](https://docs.rs/stderr/)  
or 
* Run `cargo doc --open` after modified the toml file.

## Example

```rust
#[macro_use] extern crate stderr;

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

    //If you need to use `dbxxx`,you should run `init!()` or `Loger::init(module_path!())` before use them on current process.
    use stderr::log::*;
    Logger::init(pkg!()); 
    dbln!();
    dbln!("db!/dbln!()@Loger !");
    db!("{}\n", s);
    dbln!("{:?}", vec);

    dbstln!();
    dbstln!("dbst!/dbstln!()@Loger !");
    dbst!("{}\n", s);
    dbstln!("{:?}", vec);
}
```
## ChangLog
2017-0508 **0.8.0** `log::*`

2017-0424 **0.7.1** `loc!()` and `Loger::init(module_path!())`

2017-0211 **0.7.0** Supports multiple packages.

2017-0126 **0.6.1** Don't repeat initialization,fix #2.

2017-0126 **0.6.0** Refactoring and update API to `stderr::Loger; init!(),db!(),dbln!(),dbst!(),dbstln!()` for `LOG=module_path!()` and `--log/-log module_path!()`,add Documentation.

2017-0116 __0.5.0__ Add `stderr::loger::Loger;init(), debug!(),debugln!(),debugst!(),debugstln!()` for `LOG=debug` and `--log/-log debug`.
