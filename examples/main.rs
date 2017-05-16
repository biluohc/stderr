#[macro_use]
extern crate stderr;
use stderr::log::*;

fn main() {
    init!();
    db!("x");
    println!();
    db!("x {}", 1);
    println!();
    db!("x {}/{}", 1, 2);
    println!();
    dbln!();
    dbln!("x_ln");
    dbln!("x_ln {}", 1);
    dbln!("x_ln {}/{}", 1, 2);
    y::ffmpeg();
}

mod y {
    use stderr::log::*;
    pub fn ffmpeg() {
        Logger::init(pkg!());
        println!();
        db!("y");
        println!();
        db!("y {}", 1);
        println!();
        db!("y {}/{}", 1, 2);
        println!();
        dbln!();
        dbln!("y_ln");
        dbln!("y_ln {}", 1);
        dbln!("y_ln {}/{}", 1, 2);
        z::ffmpeg();
        z1::ffmpeg();
    }
    mod z {
        use stderr::log::*;
        pub fn ffmpeg() {
            println!();
            db!("z");
            println!();
            db!("z {}", 1);
            println!();
            db!("z {}/{}", 1, 2);
            println!();
            dbln!();
            dbln!("z_ln");
            dbln!("z_ln {}", 1);
            dbln!("z_ln {}/{}", 1, 2);
        }
    }
    mod z1 {
        use stderr::log::*;
        pub fn ffmpeg() {
            println!();
            dbst!("st_z1");
            println!();
            dbst!("st_z1 {}", 1);
            println!();
            dbst!("st_z1 {}/{}", 1, 2);
            println!();
            dbstln!();
            dbstln!("st_z1_ln");
            dbstln!("st_z1_ln {}", 1);
            dbstln!("st_z1_ln {}/{}", 1, 2);
        }
    }
}
