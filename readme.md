# A Rust's library that using macro to write io::stderr() for error or io::stdout() for log optional.

## Usage
Cargo.toml

```toml
[dependencies]
stderr = "0.5.0"
```
or
```toml
[dependencies]
stderr = { git = "https://github.com/biluohc/stderr", branch = "master", version = "0.5.0"}
```

## Explain
### About stderr
Usage as same as print!/println!.

1. `err!`/`errln!`: Panics if writing to `io::stdout()` fails.
2. `errst!`/`errstln!`: Do nothing if writing to `io::stdout()` fails(silent->st).

### About stderr::loger::Loger
Avoid to note or use a bunch of `print!()/println!()` non-stop.  
`debug!()`/`debugln!()` print message while args conntains `-log` or `--log` and `debug` follows it,or environment variable 'LOG' == `debug`.  
Usage as same as `err!`/`errln!` and `errst!`/`errstln!`.

Example for bash:
```bash
    env LOG=debug  your_exe_file_path --log debug
```  
If you neend to use `debug!()`/`debugln!()`:
You must use `Loger::init()` before use the macro on the current process.  
if you need to contrl it:  
You can use `Loger::set(bool)` to replace `Loger::init()`,  
`Loger::arg()`: Get args's `-log/--log`==`debug` value(bool),  
`Loger::var()`: Get environment variable's `LOG`==`debug` value(bool).  

## Example

```rust
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

    use stderr::loger::Loger;
    Loger::init();
    debugln!();
    debugln!("debug!/debugln!()@Loger !");
    debug!("{}\n", s);
    debugln!("{:?}", vec);

    debugstln!();
    debugstln!("debugst!/debugstln!()@Loger !");
    debugst!("{}\n", s);
    debugstln!("{:?}", vec);
}
```

