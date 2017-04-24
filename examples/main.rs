#[macro_use]
extern crate stderr;
use stderr::Loger;

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
    use super::Loger;
    pub fn ffmpeg() {
        Loger::init(module_path!());
        Loger::with_time(true);
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
        Loger::with_time(false);        
        z::ffmpeg();
        z1::ffmpeg();
    }
    mod z {
        use super::Loger;
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
        use super::Loger;
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