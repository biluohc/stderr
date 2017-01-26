use std::env::args;
use std::env::var;
use std::slice;
use std::str;

#[derive(Debug)]
pub struct Loger {
    // mod 路径 。通过init!()初始化(默认只打印当前 crate 的)，..用参数/环境变量设置
    ptr: *const u8,
    len: usize,
    // log状态,默认关闭。
    status: bool,
}
static mut LOGER: Loger = Loger {
    ptr: 0 as *const u8,
    len: 0,
    status: false,
};

const KEY_VAR: &'static str = "LOG"; // env LOG=*
const KEY_ARG: &'static str = "log"; // exe -log[--log]

impl Loger {
    pub fn set(module_path: &str) {
        let (mut mp, mut status) = (module_path.to_owned(), false);

        // 取环境变量
        if let Ok(ok) = var(KEY_VAR) {
            mp = ok;
            status = true;
        }
        //环境变量为空则取命令行参数
        if !status {
            let args_vec: Vec<String> = args().collect();
            let mut idx = 0;
            for _ in 0..args_vec.len() {
                if args_vec[idx] == format!("-{}", KEY_ARG) ||
                   args_vec[idx] == format!("--{}", KEY_ARG) {
                    //如果无值，开启crate的log.
                    status = true;
                    if idx + 1 < args_vec.len() {
                        mp = args_vec[idx + 1].clone();
                    }
                    break;
                }
                idx += 1;
            }
        }
        unsafe {
            LOGER.ptr = mp.as_str().as_ptr();
            LOGER.len = mp.len();
            LOGER.status = status;
        }
    }
    pub fn get() -> String {
        unsafe { String::from_utf8_lossy(slice::from_raw_parts(LOGER.ptr, LOGER.len)).into_owned() }
    }
    pub fn status() -> bool {
        unsafe { LOGER.status }
    }

    // #[doc(hidden)]
    pub fn mp_parse(module_path: &'static str) -> bool {
        let refer = Self::get();
        //"*"匹配的全部。
        if refer == "*" {
            return true;
        }
        //完全匹配,避免后面切片失败。
        if module_path == refer {
            return true;
        }
        //属于该模块。
        if module_path.starts_with(&refer) && (&module_path[refer.len()..]).starts_with(':') {
            return true;
        }
        false
    }
}

#[macro_export]
macro_rules! init {
    () => {
        let msg= module_path!();
        //fht2p::server::args::app -> fht2p
        let sep_idx= msg.find(":").unwrap_or(msg.len()) ;
        Loger::set(&msg[..sep_idx] );
    };
}
#[macro_export]
macro_rules! db {
    ($($arg:tt)*) => {
        {     
             if Loger::status() &&Loger::mp_parse(module_path!()) {
                use std::io::{self, Write};
                // Panics if writing to io::stdout() fails.                
                write!(&mut io::stderr(),$($arg)*).unwrap();
         }
        }
        };
}
#[macro_export]
macro_rules! dbln { 
       () => (db!("\n"));
       ($fmt:expr) => (db!(concat!($fmt, "\n")));
        ($fmt:expr, $($arg:tt)*) => (db!(concat!($fmt, "\n"), $($arg)*));
}
// dbst!,dbstln!
#[macro_export]
macro_rules! dbst {
    ($($arg:tt)*) => {
        { 
         if Loger::status() &&Loger::mp_parse(module_path!()) {
                    use std::io::{self, Write};
                    if let Ok(..) =  write!(&mut io::stderr(),$($arg)* ) {};
                    // Do nothing if writing to io::stdout() fails(silent->st).
              }
        }
        };
}
#[macro_export]
macro_rules! dbstln {
       () => (dbst!("\n"));
       ($fmt:expr) => (dbst!(concat!($fmt, "\n")));
        ($fmt:expr, $($arg:tt)*) => (dbst!(concat!($fmt, "\n"), $($arg)*));
        // Do nothing if writing to io::stdout() fails(silent->st).
}
