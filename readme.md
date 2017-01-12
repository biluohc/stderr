# A library that using macro to write to io::stderr() for Rust.

## Usage
Cargo.toml

```toml
[dependencies]
stderr = "0.3.0"
```

or

```toml
[dependencies]
stderr = { git = "https://github.com/biluohc/stderr", branch = "master", version = "0.3.0"}
```

## Explain
Usage as same as print!/println!.

1. `err!`/`errln!`: Panics if writing to `io::stdout()` fails.
2. `errst!`/`errstln!`: Do nothing if writing to `io::stdout()` fails(silent->st).

## Example

```rust
#[macro_use]
extern crate stderr;

fn main() {
    println!("stderr@stdout !");
    let vec = vec![1, 2, 3, 4, 5];
    let s = std::env::args().nth(0).unwrap();

    err!("err!(expr\\n)\n");
    err!("err!(expr,tt) {}\t{:?}", s, vec);
    errln!();
    errln!("errln!(expr)");
    for (i, x) in vec.clone().iter().enumerate() {
        errln!("errln!(expr,tt) {}: {}", i, x);
    }
    println!();

    errst!("errst!(expr\\n)\n");
    errst!("errst!(expr,tt) {}\t{:?}", s, vec);
    errstln!();
    errstln!("errstln!(expr)");
    for (i, x) in vec.iter().enumerate() {
        errstln!("errstln!(expr,tt) {}: {}", i, x);
    }
}
```
