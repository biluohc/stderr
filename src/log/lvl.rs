/// Log Level
#[derive(Debug,PartialEq,PartialOrd)]
pub enum LogLvl {
    ///`0`
    ///
    ///`Logger::close()`
    Off,
    ///`1`
    ///
    ///`fatal!()`, `fataln!()`
    Fatal,
    ///`2`
    ///
    ///`error!()`, `errorln!()`
    Error,
    ///`3`
    ///
    ///`warn!()`, `warnln!()`
    Warn,
    ///`4`
    ///
    ///`info!()`, `infoln!()`
    Info,
    ///`5`
    ///
    ///`db!()` and `debug`, `dbln!()` and `debugln!()`
    Debug,
    ///`6`
    ///
    ///`Logger::open()`
    All,
}
impl Default for LogLvl {
    fn default() -> Self {
        LogLvl::Info
    }
}

impl LogLvl {
    /// `0~6` for `i8 u8 i16 u16 i32 u32 i64 u64 isize usize`
    pub fn form_num<T: IsInterger>(num: T) -> Option<Self> {
        Self::from_str(&format!("{}", num))
    }
    ///`
    /// "Off Fatal Error Warn Info Debug All 0 1 2 3 4 5 6"
    ///.split(' ').map(|s|s.trim().to_string())
    ///`
    #[allow(unknown_lints,should_implement_trait)]
    pub fn from_str(str: &str) -> Option<Self> {
        match str.to_lowercase().as_str() {  
            "off" | "0" => Some(LogLvl::Off),
            "fatal" | "1" => Some(LogLvl::Fatal),
            "error" | "2" => Some(LogLvl::Error),
            "warn" | "3" => Some(LogLvl::Warn),
            "info" | "4" | "" => Some(LogLvl::Info),
            "debug" | "5" => Some(LogLvl::Debug),
            "all" | "6" | "*" => Some(LogLvl::All),
            _ => None, 
        }
    }
}

impl Display for LogLvl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match *self {
            LogLvl::Off => "Off",
            LogLvl::Fatal => "Fatal",
            LogLvl::Error => "Error",
            LogLvl::Warn => "Warn",
            LogLvl::Info => "Info",
            LogLvl::Debug => "Debug",
            LogLvl::All => "All",
        };
        write!(f, "{}", str)
    }
}
use std::fmt;
use std::fmt::Display;
pub trait IsInterger: Display {}
macro_rules! is_interger {
        ($($t:ty)*) => ($(
        impl IsInterger for $t {}
    )*)
}
is_interger! {i8 u8 i16 u16 i32 u32 i64 u64 isize usize}

#[test]
fn lvl() {
    const LogLvlStr: &'static str = "Off Fatal Error Warn Info Debug All 0 1 2 3 4 5 6";
    let nums: Vec<LogLvl> = (0..7).map(|s| LogLvl::form_num(s).unwrap()).collect();
    println!("{:?}", nums);
    let strs: Vec<LogLvl> = LogLvlStr
        .split(" ")
        .filter(|s| !s.trim().is_empty())
        .map(|s| LogLvl::from_str(s).unwrap())
        .collect();
    println!("{:?}", strs);
    let strss: Vec<LogLvl> = LogLvlStr
        .split(" ")
        .filter(|s| !s.trim().is_empty())
        .map(|s| LogLvl::from_str(s.trim()).unwrap())
        .collect();
    println!("{:?}", strss);
    assert_eq!(&nums[..], &strs[..7]);
    assert_eq!(&nums[..], &strs[7..]);
    assert_eq!(&nums[..], &strss[..7]);
    assert_eq!(&nums[..], &strss[7..]);
    for i in 0..nums.len() {
        print!("{}\n", i);
        println!("{:?}=={:?}: {}", nums[i], strs[i], nums[i] == strs[i]);
        assert!(nums[i] == strs[i]);
        for ii in &nums[0..i] {
            println!("{:?}>{:?}: {}", nums[i], ii, nums[i] > *ii);
            assert!(nums[i] > *ii);
        }
        for ii in &nums[i + 1..] {
            println!("{:?}<{:?}: {}", nums[i], ii, nums[i] < *ii);
            assert!(nums[i] < *ii);
        }
        println!();
    }
}
