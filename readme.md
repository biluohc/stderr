# A library that using macro to write io::stderr() like print!/println! for Rust.

## Usage
Cargo.toml

    [dependencies] 
    stderr = "0.2.0"
or   
    
    [dependencies] 
    stderr = { git = "https://github.com/biluohc/stderr" }  

## Explain 
Usage as same as print!/println!.  
1. err!/errln!: Panics if writing to io::stdout() fails.  
2. errst!/errstln!: Do nothing if writing to io::stdout() fails(silent->st). 

## Example  

``` 
#[macro_use]
extern crate stderr;

fn main() {
    println!("stdout@stderr !");
    let vec = vec![1, 2, 3, 4, 5];
    let s = std::env::args().nth(0).unwrap();

    err!("stderr::err!(expr\\n)\n");
    err!("stderr::err!(expr,tt) {}\t{:?}", s, vec);
    errln!();
    errln!("stderr::errln!(expr)");
    for (i, x) in vec.clone().iter().enumerate() {
        errln!("stderr::errln!(expr,tt) {}: {}", i, x);
    }
    println!();

    errst!("stderr::errst!(expr\\n)\n");
    errst!("stderr::errst!(expr,tt) {}\t{:?}", s, vec);
    errstln!();
    errstln!("stderr::errstln!(expr)");
    for (i, x) in vec.iter().enumerate() {
        errstln!("stderr::errstln!(expr,tt) {}: {}", i, x);
    }
}  
