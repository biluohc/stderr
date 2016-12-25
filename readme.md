# A stderr library written for Rust.

## Usage
Cargo.toml

    [dependencies] 
    stderr = { git = "https://github.com/biluohc/stderr" }  

## Explain 
* Usage as same as print!/println!.
* stderr!/stderrln!: Writing to standard error failed will panic this thread.
* stderr_qt!/stderr_qtln!: Writing to standard error failed will do nothing.

## Example  
    #[macro_use]
    extern crate stderr;
    use stderr::*;

    fn main() {
        println!("Hello, world!");
        println!("stdout@stderr !");
        let vec = vec![1, 2, 3, 4, 5];
        let s = std::env::args().nth(0).unwrap();
        stderr!("stderr! {}\t{:?}", s, vec);
        stderrln!("stderrln! {}", s);
        stderr_qt!("stderr_qt! {}\t{:?}", s, vec);
        stderr_qtln!("stderr_qtln! {}", s);
    }
