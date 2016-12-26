#[macro_use]
mod lib;

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