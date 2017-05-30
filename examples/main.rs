#[macro_use]
extern crate stderr;

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
    stderr::log::LogFmter::set(stderr::log::fmter_with_time);
    y::ffmpeg();
}

mod y {
    pub fn ffmpeg() {
        logger_init!();
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
            info!("info\n");
            infoln!("infoln");
            warn!("warn\n");
            warnln!("warnln");
            error!("error\n");
            errorln!("errorln");
        }
    }
}
