/**
Get `crate` name

```rustfull
println!("{}",pkg!()); // -> www
```
*/
#[macro_export]
macro_rules! pkg {
    () => {
        {let mp = module_path!();
        if mp.contains(':') {
                &mp[0..mp.find(':').unwrap()]
        }else {
               mp
        }
        }
    }
}

/** Get location(`module_path:line:column`)

```rustful
println!("{}",loc!());    // module_path!():line!():clumn!() -> www::16::23
println!("{:?}",loc!());  // LogLoc {...}
```
*/
#[macro_export]
macro_rules! loc {
    () => (
    $crate::log::LogLoc::new(module_path!(),line!(), column!(),file!())
    )
}

/// Equal to `logger_init!()`
#[deprecated(since = "0.7.1",note = "Should use `logger_init!()` instead")]
#[macro_export]
macro_rules! init {
    () => {
        logger_init!();
    };
}

/// Equal to `Logger::init(pkg!())`
#[macro_export]
macro_rules! logger_init {
    () => {
        $crate::log::Logger::init(pkg!());
    };
}

#[macro_export]
macro_rules! db {
        ($($arg:tt)*) => (
                if $crate::log::Logger::enable() && $crate::log::Logger::filter(module_path!(),$crate::log::LogLvl::Debug) {
                use std::io::{self, Write};
                // Panics if writing to io::stdout() fails.           
               &mut io::stderr().write_all($crate::log::LogMsg::new(loc!(),format_args!($($arg)*),$crate::log::LogLvl::Debug).call().as_bytes()).unwrap();
                }
        );
}

/// Equal to `db!()`
#[macro_export]
macro_rules! debug {
        ($($arg:tt)*) => (
            db!($($arg)*)
        );
}
#[macro_export]
macro_rules! dbln { 
       () => (db!("\n"));
       ($fmt:expr) => (db!(concat!($fmt, "\n")));
       ($fmt:expr, $($arg:tt)*) => (db!(concat!($fmt, "\n"), $($arg)*));
}
/// Equal to `dbln!()`
#[macro_export]
macro_rules! debugln {
        ($($arg:tt)*) => (
            dbln!($($arg)*)
        );
}

#[macro_export]
macro_rules! dbst {
        ($($arg:tt)*) => (
                if $crate::log::Logger::enable() &&$crate::log::Logger::filter( module_path!(),$crate::log::LogLvl::Debug) {
                use std::io::{self, Write};
                    // Do nothing if writing to io::stdout() fails(silent->st).
               let _= &mut io::stderr().write_all($crate::log::LogMsg::new(loc!(),format_args!($($arg)*),$crate::log::LogLvl::Debug).call().as_bytes());
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

/// write the `LogMsg` to `stderr` and `panic` with the `LogMsg`
#[macro_export]
macro_rules! fatal {
        ($($arg:tt)*) => (
                use std::io::{self, Write};                
                let log_str = $crate::log::LogMsg::new(loc!(),format_args!($($arg)*),$crate::log::LogLvl::Fatal).call();
                if $crate::log::Logger::enable() &&$crate::log::Logger::filter(module_path!(),$crate::log::LogLvl::Fatal) {
                &mut io::stderr().write_all(log_str.as_bytes()).unwrap();
                }
                panic!("{}",log_str);                
        );
}

/// write the `LogMsg` to `stderr` and `panic` with the `LogMsg`
#[macro_export]
macro_rules! fataln { 
       () => (fatal!("\n"));
       ($fmt:expr) => (fatal!(concat!($fmt, "\n")));
       ($fmt:expr, $($arg:tt)*) => (fatal!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! error {
        ($($arg:tt)*) => (
                if $crate::log::Logger::enable() &&$crate::log::Logger::filter(module_path!(),$crate::log::LogLvl::Error) {
                use std::io::{self, Write};
                // Panics if writing to io::stdout() fails.           
               &mut io::stderr().write_all($crate::log::LogMsg::new(loc!(),format_args!($($arg)*),$crate::log::LogLvl::Error).call().as_bytes()).unwrap();
                }
        );
}

#[macro_export]
macro_rules! errorln { 
       () => (error!("\n"));
       ($fmt:expr) => (error!(concat!($fmt, "\n")));
       ($fmt:expr, $($arg:tt)*) => (error!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! warn {
        ($($arg:tt)*) => (
                if $crate::log::Logger::enable() &&$crate::log::Logger::filter(module_path!(),$crate::log::LogLvl::Warn) {
                use std::io::{self, Write};
                // Panics if writing to io::stdout() fails.           
               &mut io::stderr().write_all($crate::log::LogMsg::new(loc!(),format_args!($($arg)*),$crate::log::LogLvl::Warn).call().as_bytes()).unwrap();
                }
        );
}

#[macro_export]
macro_rules! warnln { 
       () => (warn!("\n"));
       ($fmt:expr) => (warn!(concat!($fmt, "\n")));
       ($fmt:expr, $($arg:tt)*) => (warn!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! info {
        ($($arg:tt)*) => (
                if $crate::log::Logger::enable() &&$crate::log::Logger::filter(module_path!(),$crate::log::LogLvl::Info) {
                use std::io::{self, Write};
                // Panics if writing to io::stdout() fails.           
               &mut io::stderr().write_all($crate::log::LogMsg::new(loc!(),format_args!($($arg)*),$crate::log::LogLvl::Info).call().as_bytes()).unwrap();
                }
        );
}

#[macro_export]
macro_rules! infoln { 
       () => (info!("\n"));
       ($fmt:expr) => (info!(concat!($fmt, "\n")));
       ($fmt:expr, $($arg:tt)*) => (info!(concat!($fmt, "\n"), $($arg)*));
}
