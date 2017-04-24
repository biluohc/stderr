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

    use stderr::Loger;
    Loger::init(module_path!()); //If you need to use dbxxx,you should run `Loger::init()` before use them on current process.
    dbln!();
    dbln!("db!/dbln!()@Loger !");
    db!("{}\n", s);
    dbln!("{:?}", vec);

    dbstln!();
    dbstln!("dbst!/dbstln!()@Loger !");
    dbst!("{}\n", s);
    dbstln!("{:?}", vec);
}
