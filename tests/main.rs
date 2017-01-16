#[macro_use]
extern crate stderr;

#[test]
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
