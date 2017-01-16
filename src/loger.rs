static mut LOG: bool = false;
// 可以通过arg(),var(),set()去控制接收命令行参数或环境变量的行为。
// 以后可以换为字符整数（"0"，"1","2"...）或其对应枚举（Lv::debug,Lv::info..）去支持级别(level)。

#[derive(Debug)]
pub struct Loger {
// key_value 不能放在结构体里，否则一离开作用域就访问不到了，
}
static KEY: &'static str = "log"; //arg="-log" or "--log",env var="LOG"
static VALUE: &'static str = "debug"; // while "debug","DEBUG",.. to print debug!()

impl Loger {
    pub fn set(log: bool) {
        unsafe {
            LOG = log;
        }
    }
    pub fn log() -> bool {
        unsafe { LOG }
    }
    pub fn init() {
        // println!("Loger::init(): {}", Self::log());
        // default is false,so to update LOG if LOG==false
        if !Self::log() {
            // println!("Loger::arg()_in: {}", Self::log());
            Self::set(Self::var());
            // println!("Loger::arg()_out: {}", Self::log());
        }
        if !Self::log() {
            // println!("Loger::var()_in: {}", Self::log());
            Self::set(Self::arg());
            // println!("Loger::var()_out: {}", Self::log());
        }
    }
    pub fn var() -> bool {
        let key = &KEY.to_uppercase(); // LOG
        let value = VALUE.to_lowercase(); //debug
        use std::env::var;
        if let Ok(ok) = var(key) {
            // println!("Loger::var(): {}/{}/{}", ok.to_lowercase(), key, value);
            return ok.to_lowercase() == value;
        }
        false
    }
    pub fn arg() -> bool {
        let key_short = &format!("-{}", KEY.to_lowercase());// -log
        let key_long = &format!("--{}", KEY.to_lowercase());// --log
        let value = VALUE.to_lowercase(); //debug
        use std::env::args;
        let args: Vec<String> = args().collect();
        for i in 0..args.len() {
            let arg = &args[i];
            // println!("Loger::arg(): i/args.len()-1={}/{}", i, args.len() - 1);
            // println!("Loger::arg(): key_short='{}', key_long='{}', value='{}', arg='{}'",
            //          key_short,
            //          key_long,
            //          value,
            //          arg);
            if (arg == key_long || arg == key_short) && args.len() > i + 1 {
                let s = (&args[i + 1]).to_owned().to_lowercase();
                if s == value {
                    return true;
                }
            }
        }
        false
    }
}

// debug!,debugln!
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        {
            // 这样不适用时没有多少开销（每次调用宏次取得全局变量并判断）.
            // 也避免了不停的注销启用一堆print!().
         if Loger::log() {
            use std::io::{self, Write};
            write!(&mut io::stdout(),$($arg)*).unwrap();
         }
        }
        };
}
#[macro_export]
macro_rules! debugln { 
       () => (debug!("\n"));
       ($fmt:expr) => (debug!(concat!($fmt, "\n")));
        ($fmt:expr, $($arg:tt)*) => (debug!(concat!($fmt, "\n"), $($arg)*));
        // Panics if writing to io::stdout() fails.
}

// dbst!,dbstln!
#[macro_export]
macro_rules! debugst {
    ($($arg:tt)*) => {
        {
         if Loger::log() {
        use std::io::{self, Write};
        if let Ok(..) =  write!(&mut io::stdout(),$($arg)* ) {};
        // Do nothing if writing to io::stdout() fails(silent->st).
         }
        }
        };
}
#[macro_export]
macro_rules! debugstln {
       () => (debugst!("\n"));
       ($fmt:expr) => (debugst!(concat!($fmt, "\n")));
        ($fmt:expr, $($arg:tt)*) => (debugst!(concat!($fmt, "\n"), $($arg)*));
        // Do nothing if writing to io::stdout() fails(silent->st).
}
