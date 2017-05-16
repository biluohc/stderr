#![allow(non_upper_case_globals)]
#[macro_use]
mod macros;
mod lvl;
use self::lvl::LogLvlStr;
pub use self::lvl::LogLvl;

use time::{now, Tm};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::BTreeSet as Set;
use std::collections::btree_set::Iter;
use std::env::var;
const KEY_VAR: &'static str = "LOG"; // env LOG=*
//  set -x LOG "Info/"
//  set -x LOG "/fht2p"
//  set -x  LOG "Debug/fht2p::main,fht2p::main::{server,args}"

// init_by_args() level/mods
// init_by_set()  level(num)/mods
lazy_static!{
    static ref LOGGER:OnceInit<Logger>=OnceInit::new(Logger::default());
    static ref LogLvlStrs: Set<String> = LogLvlStr.split(' ')
    .filter(|s|!s.trim().is_empty())
    .map(|s|s.trim().to_string()).collect();
    static ref LogFmterDefault:OnceInit<LogFmter> =OnceInit::new(LogFmter::default());
}

/// Logger
#[derive(Debug,Default)]
pub struct Logger {
    initialized: AtomicBool,
    enabled: AtomicBool,
    max_lvl: LogLvl,
    mod_paths: Set<String>,
}

impl Logger {
    pub fn open() {
        LOGGER.as_ref().enabled.store(true, Ordering::SeqCst);
    }
    pub fn close() {
        LOGGER.as_ref().enabled.store(false, Ordering::SeqCst);
    }
    pub fn enable() -> bool {
        LOGGER.as_ref().enabled.load(Ordering::Relaxed)
    }
    pub fn max_lvl() -> &'static LogLvl {
        &LOGGER.as_ref().max_lvl
    }
    /// `mod_path[s]`
    pub fn mps() -> Iter<'static, String> {
        LOGGER.as_ref().mod_paths.iter()
    }
    /// The current time in the local timezone
    pub fn now() -> Tm {
        now()
    }
    /// `Logger` init by `crate_name` and `env::var("LOG")`,then `Logger::open()`
    ///
    /// `Logger::init(pkg!());` or `logger_int!()`
    ///
    /// **Notice**: `Logger` only init once time, other will be ignored.
    pub fn init(crate_name: &'static str) {
        Self::init_with_str(crate_name, var(KEY_VAR).ok());
    }
    /// `Logger` init by `crate_name` and `env_str`,then `Logger::open()`
    ///
    ///`Logger::init_with_str(pkg!(),Some("fht2p"));`
    ///
    /// **Notice**: `Logger` only init once time, other will be ignored.
    pub fn init_with_str<S: Into<String>>(mut crate_name: &'static str, env_var: Option<S>) {
        {
            let mut logger = LOGGER.as_mut();
            // avoid init second
            if logger.initialized.load(Ordering::Relaxed) {
                return;
            }
            logger.initialized.store(true, Ordering::SeqCst);
            // Compatible with previous ``Logger::init(module_path!());
            if crate_name.contains(':') {
                let sep_idx = crate_name.find(':').unwrap();
                crate_name = &crate_name[..sep_idx];
            }

            if let Some(s) = env_var {
                let s = s.into();
                let s = s.trim();
                if !s.contains('/') && !s.contains(':') && !s.contains(',') &&
                   LogLvlStrs.contains(s) {
                    // level
                    logger.max_lvl = LogLvl::from_str(s).unwrap();
                    logger.mod_paths.insert(crate_name.to_string());
                } else if s.is_empty() {
                    // set -x "" => current-crate, All-Lvl
                    logger.max_lvl = LogLvl::default();
                    logger.mod_paths.insert(crate_name.to_string());
                } else if !s.contains('/') {
                    // mod path[s]
                    logger.mod_paths = s.split(',')
                        .filter(|ss| !ss.trim().is_empty())
                        .map(|ss| ss.to_string())
                        .collect();
                } else {
                    // contains "/"
                    // "/h"[sep_idx+1..] => "h"
                    // "/"[sep_idx+1..] => ""
                    let sep_idx = s.find('/').unwrap();

                    if s.ends_with('/') {
                        // "level"
                        logger.max_lvl = LogLvl::from_str(&s[..sep_idx + 1])
                            .unwrap_or_else(LogLvl::default);
                        logger.mod_paths.insert(crate_name.to_string());
                    } else {
                        // "path[s]" || "both"
                        logger.max_lvl = LogLvl::from_str(&s[..sep_idx + 1])
                            .unwrap_or_else(LogLvl::default);
                        s[sep_idx + 1..]
                            .split(',')
                            .filter(|ss| !ss.trim().is_empty())
                            .map(|ss| logger.mod_paths.insert(ss.to_string()))
                            .count();

                    }
                }
            }
        }
        Self::open();
    }
    ///Log message occurs at current module and current LogLvl whether need output
    pub fn filter(mod_path: &str, lvl: LogLvl) -> bool {
        let logger = LOGGER.as_ref();
        if !logger.enabled.load(Ordering::Relaxed) {
            return false;
        }
        if lvl > logger.max_lvl {
            return false;
        }
        //  * match all, avoid panic because of out of index
        if logger.mod_paths.contains("*") || logger.mod_paths.contains(mod_path) {
            return true;
        }
        for path in &logger.mod_paths {
            if mod_path.starts_with(path) && (&mod_path[0..path.len() + 1]).ends_with(':') {
                return true;
            }
        }
        false
    }
}

use std::fmt::Arguments;
/// Log Location
#[derive(Debug)]
pub struct LogLoc {
    mod_path: &'static str,
    line: u32,
    column: u32,
    file: &'static str,
    time_local: Tm,
    time_utc: Tm,
}
impl LogLoc {
    /// Call it by `loc!()`
    pub fn new(mod_path: &'static str, line: u32, column: u32, file: &'static str) -> Self {
        let time_local = now();
        LogLoc {
            mod_path: mod_path,
            line: line,
            column: column,
            file: file,
            time_local: time_local,
            time_utc: time_local.to_utc(),
        }
    }
    /// `mod_path`
    pub fn mp(&self) -> &'static str {
        self.mod_path
    }
    pub fn line(&self) -> &u32 {
        &self.line
    }
    pub fn column(&self) -> &u32 {
        &self.column
    }
    pub fn file(&self) -> &'static str {
        self.file
    }
}

use std::fmt;
impl fmt::Display for LogLoc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.mp(), self.line(), self.column())
    }
}

/// Log Message
#[derive(Debug)]
pub struct LogMsg<'a> {
    loc: LogLoc,
    msg: Arguments<'a>,
    lvl: LogLvl,
}
impl<'a> LogMsg<'a> {
    pub fn new(loc: LogLoc, message: Arguments<'a>, lvl: LogLvl) -> Self {
        LogMsg {
            loc: loc,
            msg: message,
            lvl: lvl,
        }
    }
    pub fn loc(&self) -> &LogLoc {
        &self.loc
    }
    pub fn msg(&'a self) -> &'a Arguments {
        &self.msg
    }
    pub fn lvl(&'a self) -> &'a LogLvl {
        &self.lvl
    }
    /// Format `&self` by `LogFmter`
    pub fn call(&self) -> String {
        LogFmter::call(self)
    }
}

/// Log Formater
pub struct LogFmter {
    fn_: Box<Fn(&LogMsg) -> String>,
    initialized: AtomicBool,
}

impl LogFmter {
    /// Set only once
    ///
    ///`
    /// LogFmter::set(fmter);
    ///`
    pub fn set<F: IntoLogFmter>(f: F) {
        if LogFmterDefault
               .as_ref()
               .initialized
               .load(Ordering::Relaxed) {
            return;
        }
        LogFmterDefault
            .as_ref()
            .initialized
            .store(true, Ordering::SeqCst);
        let fmter = LogFmterDefault.as_mut();
        *fmter = f.into();
    }
    /// Format `&LogMesg` by `LogFmter`
    pub fn call(msg: &LogMsg) -> String {
        (LogFmterDefault.as_ref().fn_)(msg)
    }
}
fn fmter(msg: &LogMsg) -> String {
    format!("[{}!]@{}:{}:{} {}",
            msg.lvl(),
            msg.loc().mp(),
            msg.loc().line(),
            msg.loc().column(),
            msg.msg())
}
impl Default for LogFmter {
    fn default() -> LogFmter {
        LogFmter {
            fn_: (Box::new(fmter)),
            initialized: AtomicBool::new(false),
        }
    }
}

/// Format `LogMsg` by `Fn(&LogMsg) -> String + 'static + Send`
///
/// Default as follows:
///
///```rusful
///fn fmter(msg: &LogMsg) -> String {
///    format!("[{}!]@{}:{}:{} {}",
///             msg.lvl(),
///             msg.loc().mp(),
///             msg.loc().line(),
///             msg.loc().column(),
///             msg.msg())
/// }
///```
pub trait IntoLogFmter {
    fn into(self) -> LogFmter;
}

impl<F: Fn(&LogMsg) -> String + 'static + Send> IntoLogFmter for F {
    fn into(self) -> LogFmter {
        LogFmter {
            fn_: Box::new(self),
            initialized: AtomicBool::new(true),
        }
    }
}
use self::o::OnceInit;
mod o {
    use std::marker::Sync;
    use std::cell::UnsafeCell;

    #[derive(Debug)]
    pub struct OnceInit<T>(UnsafeCell<T>);
    unsafe impl<T> Sync for OnceInit<T> {}
    impl<T> OnceInit<T> {
        pub fn new(value: T) -> Self {
            OnceInit(UnsafeCell::new(value))
        }
        // init fisrt before use it
        pub fn as_mut(&self) -> &mut T {
            unsafe { self.0.get().as_mut().unwrap() }
        }
    }
    impl<T> AsRef<T> for OnceInit<T> {
        fn as_ref(&self) -> &T {
            unsafe { self.0.get().as_ref().unwrap() }
        }
    }
}
