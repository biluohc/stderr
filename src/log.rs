use std::mem::forget;
use std::env::args;
use std::env::var;
use std::slice;
use std::str;

#[derive(Debug)]
pub struct Loger {
    // mod 路径 。通过init!()初始化(默认打印所有 project 的)，..用命令行参数/环境变量设置（后者一设置，前者就不检查了）
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
// string的from_raw_parts方法这样搞不对(莫名其妙)。
const KEY_VAR: &'static str = "LOG"; // env LOG=*
const KEY_ARG: &'static str = "log"; // exe -log[--log]

impl Loger {
    pub fn set(module_path: &str) {
        let (mut mp_tmp, mut status) = (module_path.to_owned(), false);

        // 取环境变量
        if let Ok(ok) = var(KEY_VAR) {
            mp_tmp = ok;
            status = true;
        }
        //环境变量为空则取命令行参数
        if !status {
            let args_vec: Vec<String> = args().collect();
            let mut idx = 0;
            for _ in 0..args_vec.len() {
                if args_vec[idx] == format!("-{}", KEY_ARG) ||
                   args_vec[idx] == format!("--{}", KEY_ARG) {
                    //如果无值，开启当前package的log.
                    status = true;
                    if idx + 1 < args_vec.len() {
                        mp_tmp = args_vec[idx + 1].clone();
                    }
                    break;
                }
                idx += 1;
            }
        }
        // 当同一个stderr时输出当前project的db!,当好几个stderr(crate),取决于其init!()(多半会输出所有的）.
        let mp = if mp_tmp == "" {
            module_path.to_owned()
        } else {
            let mps: Vec<&str> = mp_tmp.split(',').map(|s| s.trim()).filter(|s| s != &"").collect();
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

        let ptr = mp.as_str().as_ptr();
        let len = mp.len();
        forget(mp);
        unsafe {
            LOGER.ptr = ptr;
            LOGER.len = len;
            LOGER.status = status;
        }
    }

    #[inline]
    pub fn get() -> String {
        unsafe { String::from_utf8_lossy(slice::from_raw_parts(LOGER.ptr, LOGER.len)).into_owned() }
    }

    #[inline]
    pub fn status() -> bool {
        unsafe { LOGER.status }
    }

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
            if module_path.starts_with(&mp) && (&module_path[mp.len()..]).starts_with(":") {
                return true;
            }
        }
        false
    }
}

#[macro_export]
macro_rules! init {
    () => {
        //避免重复初始化。
        if !Loger::status() {
        let msg= module_path!();
        //fht2p::server::args::app -> fht2p
        let sep_idx= msg.find(":").unwrap_or(msg.len()) ;
        Loger::set(&msg[..sep_idx] );}
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
