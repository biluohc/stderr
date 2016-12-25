#[macro_use]
mod lib;
use lib as stderr;

use stderr::*;

fn main() {
    println!("stdout@stderr !");
    let vec = vec![1, 2, 3, 4, 5];
    let s = std::env::args().nth(0).unwrap();
    stderr!("stderr! {}\t{:?}", s, vec);
    stderrln!("stderrln! {}", s);
    stderr_qt!("stderr_qt! {}\t{:?}", s, vec);
    stderr_qtln!("stderr_qtln! {}", s);
}