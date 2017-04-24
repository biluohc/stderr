use time::now;

use std::mem::forget;
use std::env::args;
use std::env::var;
use std::slice;
use std::str;

///`Loger`'s setting(inner `static mut`)
#[derive(Debug)]
pub struct Loger {
    // mod 路径 。通过init()初始化(默认打印第一个init的crate的)，..用环境变量/命令行参数设置（前者一设置，前者就不检查了）
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
static mut WITH_TIME: bool = false; // env LOG=*
const KEY_VAR: &'static str = "LOG"; // env LOG=*
const KEY_ARG: &'static str = "log"; // exe -log[--log]

impl Loger {
    /// set module_path
    pub fn set(module_path: &str) {
        let (mut mp_tmp, mut status) = (module_path.to_owned(), false);

        // get env::var
        if let Ok(ok) = var(KEY_VAR) {
            mp_tmp = ok;
            status = true;
        }
        //环境变量为空则取命令行参数
        if !status {
            let args_vec: Vec<String> = args().skip(1).collect();
            for idx in 0..args_vec.len() {
                if args_vec[idx] == format!("-{}", KEY_ARG) ||
                   args_vec[idx] == format!("--{}", KEY_ARG) {
                    //如果无值，开启当前package的log.
                    status = true;
                    if idx + 1 < args_vec.len() {
                        mp_tmp = args_vec[idx + 1].clone();
                    }
                    break;
                }
            }
        }
        // 当同一个stderr时输出当前project的db!,当好几个stderr(crate),取决于其init!()(多半会输出所有的）.
        let mp = if mp_tmp == "" {
            module_path.to_owned()
        } else {
            let mps: Vec<&str> = mp_tmp
                .split(',')
                .map(|s| s.trim())
                .filter(|s| s != &"")
                .collect();
            // println!("Loger::set()mps: {:?}", mps);
            let mut mps = mps.iter().fold(String::new(), |new, &s| new + s + ",");
            // avoid env LOG ',,,' assert_eq!() fails
            if mps.is_empty() {
                mps = mps + module_path + ",";
            }
            assert_eq!(mps.pop(), Some(',')); //remove a ","
            mps
        };
        // println!("Loger::set()@module_path->mp_tmp->mp@Loger::status(): {:?}->{:?}->{:?}@{}",
        //          module_path,
        //          mp_tmp,
        //          mp,
        //          status);

        let ptr = mp.as_ptr();
        let len = mp.len();
        forget(mp);
        unsafe {
            LOGER.ptr = ptr;
            LOGER.len = len;
            LOGER.status = status;
        }
    }
    /// handle `module_path` and  call `set()`
    ///
    ///  `Loger::init(module_path!())`
    pub fn init(module_path: &str) {
        if !Loger::status() {
            //fht2p::server::args::app -> fht2p
            let sep_idx = module_path
                .find(':')
                .unwrap_or_else(|| module_path.len());
            Loger::set(&module_path[..sep_idx]);
        }
    }
    /// `now()`/`loc!()` will contains time
    pub fn with_time(with_time: bool) {
        unsafe {
            WITH_TIME = with_time;
        }
    }

    /// get `module_path` setting
    #[inline]
    pub fn get() -> String {
        unsafe { String::from_utf8_lossy(slice::from_raw_parts(LOGER.ptr, LOGER.len)).into_owned() }
    }

    /// whether already open Loger
    #[inline]
    pub fn status() -> bool {
        unsafe { LOGER.status }
    }

/// example: `"[2017-0225 00:22:30]"`
    pub fn now() -> String {
        let local = now().to_local();
        if unsafe { WITH_TIME } {
            // "%Y-%m%d %H:%M:%S" => 2017-0225 00:22:30
            format!("[{:04}-{:02}{:02} {:02}:{:02}:{:02}] ",
                    local.tm_year + 1900,
                    local.tm_mon + 1,
                    local.tm_mday,
                    local.tm_hour,
                    local.tm_min,
                    local.tm_sec)
        } else {
            String::new()
        }
    }
    /// Log message occurs at current module whether need output
    // #[doc(hidden)]
    pub fn mp_parse(module_path: &'static str) -> bool {
        // "fht2p,poolite"->["fht2p", "poolite"]
        let mps_str = Self::get();
        let mps: Vec<&str> = mps_str.split(',').collect();
        // println!("mp_parse()@setting->mps<-->module_path!(): {:?}->{:?}<-->{:?}",
        //          refer,
        //          mps,
        //          module_path);
        //"*"匹配的全部。
        if mps.contains(&"*") {
            return true;
        }
        for mp in mps {
            //完全匹配,避免后面切片失败。
            if module_path == mp {
                return true;
            }
            //属于该模块。
            // println!("mp_parse()-slice: {:?}", &module_path[mp.len()..]);
            if module_path.starts_with(&mp) && (&module_path[mp.len()..]).starts_with(':') {
                return true;
            }
        }
        false
    }
}

/// Get location 
///
/// `format_args!("{}{}:{}:{}({}) ",Loger::now(),file!(),line!(),column!(),module_path!())`
#[macro_export]
macro_rules! loc {
    () => (
      format_args!("{}{}:{}:{}({}) ",Loger::now(),file!(),line!(),column!(),module_path!())
    )
}

/// equal to `Loger::init(module_path!())`
#[deprecated(since = "0.7.1",note = "Should use `Loger::init(module_path!())` instead")]
#[macro_export]
macro_rules! init {
    () => {
        Loger::init(module_path!());
    };
}
#[macro_export]
macro_rules! db {
       ($fmt:expr) => (
        if Loger::status() &&Loger::mp_parse(module_path!()) {
                use std::io::{self, Write};
                // Panics if writing to io::stdout() fails. 
                write!(&mut io::stderr(),concat!("{}", $fmt),loc!()).unwrap();
         }
       );
        ($fmt:expr, $($arg:tt)*) => (
                if Loger::status() &&Loger::mp_parse(module_path!()) {
                use std::io::{self, Write};
                // Panics if writing to io::stdout() fails. 
                write!(&mut io::stderr(), concat!("{}", $fmt),loc!(), $($arg)*).unwrap();
                }
        );
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
       ($fmt:expr) => (
        if Loger::status() &&Loger::mp_parse(module_path!()) {
                use std::io::{self, Write};
                    // Do nothing if writing to io::stdout() fails(silent->st).
                let _ = write!(&mut io::stderr(),concat!("{}", $fmt),loc!());
         }
       );
        ($fmt:expr, $($arg:tt)*) => (
                if Loger::status() &&Loger::mp_parse(module_path!()) {
                use std::io::{self, Write};
                    // Do nothing if writing to io::stdout() fails(silent->st).
               let _ =  write!(&mut io::stderr(), concat!("{}", $fmt),loc!(), $($arg)*);
                }
        );
}
#[macro_export]
macro_rules! dbstln {
       () => (dbst!("\n"));
       ($fmt:expr) => (dbst!(concat!($fmt, "\n")));
        ($fmt:expr, $($arg:tt)*) => (dbst!(concat!($fmt, "\n"), $($arg)*));
        // Do nothing if writing to io::stdout() fails(silent->st).
}
