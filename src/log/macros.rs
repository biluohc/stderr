/// Get `crate` name
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

/// @`log `   Get location(`module_path:line:column`)
///
/// `LogLoc::new(module_path!(),line!(), column!(),file!())`
#[macro_export]
macro_rules! loc {
    () => (
    LogLoc::new(module_path!(),line!(), column!(),file!())
    )
}

/// @`log `   Equal to `logger_init!()` or `Loger::init(pkg!())`
#[deprecated(since = "0.7.1",note = "Should use `logger_init!()` or `Loger::init(pkg!())` instead")]
#[macro_export]
macro_rules! init {
    () => {
        Logger::init(module_path!());
    };
}

/// @`log `   Equal to `Logger::init(pkg!())`
#[macro_export]
macro_rules! logger_init {
    () => {
        Logger::init(pkg!());
    };
}

/// @`log `
#[macro_export]
macro_rules! db {
        ($($arg:tt)*) => (
                if Logger::enable() &&Logger::filter( pkg!(),LogLvl::Debug) {
                use std::io::{self, Write};
                // Panics if writing to io::stdout() fails.           
               &mut io::stderr().write_all(LogMsg::new(loc!(),format_args!($($arg)*),LogLvl::Debug).call().as_bytes()).unwrap();
                }
        );
}

/// @`log `   Equal to `db!()`
#[macro_export]
macro_rules! debug {
        ($($arg:tt)*) => (
            db!($($arg)*)
        );
}
/// @`log `
#[macro_export]
macro_rules! dbln { 
       () => (db!("\n"));
       ($fmt:expr) => (db!(concat!($fmt, "\n")));
       ($fmt:expr, $($arg:tt)*) => (db!(concat!($fmt, "\n"), $($arg)*));
}
/// @`log `   Equal to `dbln!()`
#[macro_export]
macro_rules! debugln {
        ($($arg:tt)*) => (
            dbln!($($arg)*)
        );
}

/// @`log `   dbst!,dbstln!
#[macro_export]
macro_rules! dbst {
        ($($arg:tt)*) => (
                if Logger::enable() &&Logger::filter( pkg!(),LogLvl::Debug) {
                use std::io::{self, Write};
                    // Do nothing if writing to io::stdout() fails(silent->st).
               let _= &mut io::stderr().write_all(LogMsg::new(loc!(),format_args!($($arg)*),LogLvl::Debug).call().as_bytes());
                }
        );
}

/// @`log `
#[macro_export]
macro_rules! dbstln {
       () => (dbst!("\n"));
       ($fmt:expr) => (dbst!(concat!($fmt, "\n")));
        ($fmt:expr, $($arg:tt)*) => (dbst!(concat!($fmt, "\n"), $($arg)*));
        // Do nothing if writing to io::stdout() fails(silent->st).
}

/// @`log `
#[macro_export]
macro_rules! fatal {
        ($($arg:tt)*) => (
                if Logger::enable() &&Logger::filter(pkg!(),LogLvl::Fatal) {
                use std::io::{self, Write};
                // Panics if writing to io::stdout() fails.           
               &mut io::stderr().write_all(LogMsg::new(loc!(),format_args!($($arg)*),LogLvl::Fatal).call().as_bytes()).unwrap();
                }
        );
}

/// @`log `
#[macro_export]
macro_rules! fataln { 
       () => (fatal!("\n"));
       ($fmt:expr) => (fatal!(concat!($fmt, "\n")));
       ($fmt:expr, $($arg:tt)*) => (fatal!(concat!($fmt, "\n"), $($arg)*));
}

/// @`log `
#[macro_export]
macro_rules! error {
        ($($arg:tt)*) => (
                if Logger::enable() &&Logger::filter(pkg!(),LogLvl::Error) {
                use std::io::{self, Write};
                // Panics if writing to io::stdout() fails.           
               &mut io::stderr().write_all(LogMsg::new(loc!(),format_args!($($arg)*),LogLvl::Error).call().as_bytes()).unwrap();
                }
        );
}

/// @`log `
#[macro_export]
macro_rules! errorln { 
       () => (error!("\n"));
       ($fmt:expr) => (error!(concat!($fmt, "\n")));
       ($fmt:expr, $($arg:tt)*) => (error!(concat!($fmt, "\n"), $($arg)*));
}

/// @`log `
#[macro_export]
macro_rules! warn {
        ($($arg:tt)*) => (
                if Logger::enable() &&Logger::filter(pkg!(),LogLvl::Warn) {
                use std::io::{self, Write};
                // Panics if writing to io::stdout() fails.           
               &mut io::stderr().write_all(LogMsg::new(loc!(),format_args!($($arg)*),LogLvl::Warn).call().as_bytes()).unwrap();
                }
        );
}

/// @`log `
#[macro_export]
macro_rules! warnln { 
       () => (warn!("\n"));
       ($fmt:expr) => (warn!(concat!($fmt, "\n")));
       ($fmt:expr, $($arg:tt)*) => (warn!(concat!($fmt, "\n"), $($arg)*));
}

/// @`log `
#[macro_export]
macro_rules! info {
        ($($arg:tt)*) => (
                if Logger::enable() &&Logger::filter(pkg!(),LogLvl::Info) {
                use std::io::{self, Write};
                // Panics if writing to io::stdout() fails.           
               &mut io::stderr().write_all(LogMsg::new(loc!(),format_args!($($arg)*),LogLvl::Info).call().as_bytes()).unwrap();
                }
        );
}

/// @`log `
#[macro_export]
macro_rules! infoln { 
       () => (info!("\n"));
       ($fmt:expr) => (info!(concat!($fmt, "\n")));
       ($fmt:expr, $($arg:tt)*) => (info!(concat!($fmt, "\n"), $($arg)*));
}
