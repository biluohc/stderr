#![allow(non_upper_case_globals)]
#[macro_use]
mod macros;
mod lvl;
pub use self::lvl::LogLvl;
use super::StaticMut;

use time::{now, Tm};
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
use std::collections::BTreeSet as Set;
use std::collections::btree_set::Iter;
use std::env::var;
use std::env::args;
/// `"LOG"`
pub static mut ENV_VAR_KEY: &'static str = "LOG";
/// `["-log", "--log"]`
pub static mut CLI_OPTION_KEYS: [&'static str; 2] = ["-log", "--log"];

lazy_static!{
    static ref LOGGER:StaticMut<Logger>=StaticMut::new(Logger::default());
    static ref LogFmterDefault:StaticMut<LogFmter> =StaticMut::new(LogFmter::default());
}
static LogFmterInitialized: AtomicBool = ATOMIC_BOOL_INIT;

/// Logger
#[derive(Debug,Default)]
pub struct Logger {
    initialized: AtomicBool,
    enabled: AtomicBool,
    max_lvl: LogLvl,
    mod_paths: Set<String>,
    without_cli_options: AtomicBool,
}

impl Logger {
    /// `info/*`
    pub fn set_info_all() {
        Self::initialized_set(true);
        Self::enable_set(true);
        let mut logger = LOGGER.as_mut();
        logger.mod_paths.insert("*".to_string());
        logger.max_lvl = LogLvl::Info;
    }
    ///` Logger::enable_set(true)`
    pub fn open() {
        Self::enable_set(true);
    }
    /// `Logger::enable_set(false)`
    pub fn close() {
        Self::enable_set(false);
    }
    pub fn initialized() -> bool {
        LOGGER.as_ref().initialized.load(Ordering::Relaxed)
    }
    pub fn initialized_set(b: bool) {
        LOGGER.as_ref().initialized.store(b, Ordering::SeqCst);
    }
    pub fn enable() -> bool {
        LOGGER.as_ref().enabled.load(Ordering::Relaxed)
    }
    pub fn enable_set(b: bool) {
        LOGGER.as_ref().enabled.store(b, Ordering::SeqCst);
    }
    pub fn without_cli_options() -> bool {
        LOGGER.as_ref().without_cli_options.load(Ordering::Relaxed)
    }
    pub fn without_cli_options_set(b: bool) {
        LOGGER
            .as_ref()
            .without_cli_options
            .store(b, Ordering::SeqCst);
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
    fn cli_options(cli_option_keys: &[&'static str]) -> Option<String> {
        let mut args: Vec<String> = args().skip(1).collect();
        let idx = args.as_slice()
            .iter()
            .position(|s| cli_option_keys.iter().any(|ss| ss == &s.as_str()));
        // println!("cli_options: {:?} -> {:?}", idx, args);
        if let Some(idx) = idx {
            if args.len() >= idx + 2 {
                // println!("cli_options/args[idx+1 = {}]: {:?}",idx+1, args[idx + 1]);
                return Some(args.remove(idx + 1));
            }
        }
        None
    }
    pub fn init(crate_name: &'static str) {
        if Self::initialized() {
            return;
        }
        // println!("LOGER_before_env: {:?}\nenv::var({:?}): {:?}",
        //          LOGGER.get(),
        //          ENV_VAR_KEY,
        //          var(ENV_VAR_KEY));
        if let Ok(s) = var(unsafe { ENV_VAR_KEY }) {
            Self::init_with_str(crate_name, s);
        }
        // println!("LOGER_after_env: {:?}\ncli::cli_options({:?}): {:?}",
        //          LOGGER.get(),
        //          CLI_OPTION_KEYS,
        //          Self::cli_options(&CLI_OPTION_KEYS[..]));
        if !Self::initialized() && !Self::without_cli_options() {
            if let Some(s) = Self::cli_options(unsafe { &CLI_OPTION_KEYS[..] }) {
                Self::init_with_str(crate_name, s);
            }
        }
        if !Self::initialized() {
            Self::set_info_all();
        }
        // println!("LOGER_after_cli: {:?}", LOGGER.get());
    }
    /// `Logger` init by `crate_name` and `var`, then `Logger::open()` if `var` is valid
    ///
    ///`Logger::init_with_str(pkg!(),"fht2p");`
    ///
    /// **Notice**: `Logger` only init once time, other will be ignored.
    pub fn init_with_str<S: Into<String>>(mut crate_name: &'static str, var: S) {
        // no_input    -> info/*
        // /           -> all/pkg
        // lvl/mods    -> lvl/mods
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

        let s = var.into();
        let s = s.trim();
        let sep_idx = s.find('/');
        if sep_idx.is_none() {
            //invalid input
            return;
        }
        let sep_idx = sep_idx.unwrap();
        let (lvl_str, mut mps_str) = (&s[..sep_idx], &s[sep_idx + 1..]);
        // println!("lvl_str -> mps_str: {:?} -> {:?}", lvl_str, mps_str);
        if mps_str.is_empty() {
            mps_str = crate_name; // "" -> crate_name
        }
        // println!("{:?}", LogLvl::from_str(lvl_str));
        if let Some(lvl) = LogLvl::from_str(lvl_str) {
            logger.max_lvl = lvl;
        } else {
            return;
        }
        let mps: Vec<&str> = mps_str
            .split(',')
            .map(|ss| ss.trim().trim_matches(':'))
            .filter(|ss| !ss.is_empty())
            .collect();
        // println!("mps: {:?}", mps);
        if mps.contains(&"*") {
            logger.mod_paths.insert("*".to_string());
        } else {
            mps.into_iter()
                .map(|ss| logger.mod_paths.insert(ss.to_string()))
                .last();
        }
        Self::open();
        // println!("LOGER: {:?}", logger);
    }
    ///Log message occurs at current module and current LogLvl whether need output
    pub fn filter(mod_path: &str, lvl: LogLvl) -> bool {
        let logger = LOGGER.as_ref();
        // println!("LOGER::filter(mp: {:?},lvl: {:?}): {:?}",
        //          mod_path,
        //          lvl,
        //          logger);
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
    pub fn time_local(&self) -> &Tm {
        &self.time_local
    }
    pub fn time_utc(&self) -> &Tm {
        &self.time_utc
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
pub struct LogFmter(Box<Fn(&LogMsg) -> String>);

impl LogFmter {
    /// Set only once, default is [`fmter`](fn.fmter.html)
    ///
    ///`
    /// LogFmter::set(fmter);
    ///`
    pub fn set<F: IntoLogFmter>(f: F) {
        if LogFmterInitialized.load(Ordering::Relaxed) {
            return;
        }
        LogFmterInitialized.store(true, Ordering::SeqCst);
        LogFmterDefault.set(f.into())
    }
    /// Format `&LogMesg` by `LogFmter`
    pub fn call(msg: &LogMsg) -> String {
        (LogFmterDefault.as_ref().0)(msg)
    }
}

/// `[Debug!]#main:6:4 ..`
pub fn fmter(msg: &LogMsg) -> String {
    format!("[{}!]#{}:{}:{} {}",
            msg.lvl(),
            msg.loc().mp(),
            msg.loc().line(),
            msg.loc().column(),
            msg.msg())
}

///`[2017-05-30 13:10:00 Debug!]#main:12:8 ..`
pub fn fmter_with_time(msg: &LogMsg) -> String {
    let t = msg.loc().time_local();
    format!("[{:04}-{:02}-{:02} {:02}:{:02}:{:02} {}!]#{}:{}:{} {}",
            t.tm_year + 1900,
            t.tm_mon + 1,
            t.tm_mday,
            t.tm_hour,
            t.tm_min,
            t.tm_sec,
            msg.lvl(),
            msg.loc().mp(),
            msg.loc().line(),
            msg.loc().column(),
            msg.msg())
}

impl Default for LogFmter {
    fn default() -> LogFmter {
        LogFmter(Box::new(fmter))
    }
}

/**
 Format `LogMsg` by `Fn(&LogMsg) -> String + 'static + Send`

 Default as [`fmter`](fn.fmter.html), you can instead by [`fmter_with_time`](fn.fmter_with_time.html) or write a `fn` by youself.
*/
pub trait IntoLogFmter {
    fn into(self) -> LogFmter;
}

impl<F: Fn(&LogMsg) -> String + 'static + Send> IntoLogFmter for F {
    fn into(self) -> LogFmter {
        LogFmter(Box::new(self))
    }
}
