//! simple-log is a very simple configuration log crates.
//!
//! # simple-log format output
//!
//! ```bash
//! 2020-12-07 15:06:03:260570000 [INFO] <json_log:16>:info json simple_log
//! 2020-12-07 15:06:03:262106000 [WARN] <json_log:17>:warn json simple_log
//! 2020-12-07 15:06:03:262174000 [ERROR] <json_log:18>:error json simple_log
//! ```
//!
//! # Quick Start
//!
//! To get you started quickly, the easiest and quick way to used with demo or test project
//!
//! ```no_run
//! #[macro_use]
//! extern crate log;
//!
//! fn main() -> Result<(), String> {
//!    simple_log::quick()?;
//!
//!    debug!("test quick debug");
//!    info!("test quick info");
//!    Ok(())
//!}
//! ```
//!
//! # Usage in project
//!
//! Configuration [LogConfig] in your project.
//!
//! ```no_run
//!#[macro_use]
//!extern crate log;
//!
//!use simple_log::LogConfigBuilder;
//!
//!fn main() -> Result<(), String> {
//!    let config = LogConfigBuilder::builder()
//!        .path("./log/builder_log.log")
//!        .size(1 * 100)
//!        .roll_count(10)
//!        .level("debug")
//!        .output_file()
//!        .output_console()
//!        .build();
//!
//!    simple_log::new(config)?;
//!    debug!("test builder debug");
//!    info!("test builder info");
//!    Ok(())
//!}
//! ```
//!
//! # Config with json
//!
//! ```no_run
//! #[macro_use]
//! extern crate log;
//!
//! use simple_log::LogConfig;
//!
//! fn main() {
//!     let config = r#"
//!     {
//!         "path":"./log/tmp.log",
//!         "level":"debug",
//!         "size":10,
//!         "out_kind":"file",
//!         "roll_count":10
//!     }"#;
//!     let log_config: LogConfig = serde_json::from_str(config).unwrap();
//!
//!     simple_log::new(log_config).unwrap();//init log
//!
//!     info!("info json simple_log");
//!     warn!("warn json simple_log");
//!     error!("error json simple_log");
//! }
//! ```
//!
//! For the user guide and futher documentation, please read
//! [The simple-log document](https://github.com/baoyachi/simple-log).
//!
//! More than examples can see:
//! [examples](https://github.com/baoyachi/simple-log/tree/main/examples).
//!

mod out_kind;

use crate::out_kind::OutKind;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

type SimpleResult<T> = std::result::Result<T, String>;

#[cfg(feature = "shadow-rs")]
pub use shadow_rs::is_debug;

/// simple-log global config.
struct LogConf {
    log_config: LogConfig,
    handle: log4rs::Handle,
}

static LOG_CONF: OnceCell<Mutex<LogConf>> = OnceCell::new();

fn init_log_conf(log_config: LogConfig) -> SimpleResult<()> {
    let config = build_config(&log_config)?;
    let handle = log4rs::init_config(config).map_err(|e| e.to_string())?;
    LOG_CONF.get_or_init(|| Mutex::new(LogConf { log_config, handle }));
    Ok(())
}

/// Update simple-log global config [LogConfig].
///
/// ```rust
/// #[macro_use]
/// extern crate log;
///
/// use simple_log::LogConfigBuilder;
///
/// fn main() -> Result<(), String> {
///     let old_config = LogConfigBuilder::builder()
///         .path("./log/builder_log.log")
///         .size(1 * 100)
///         .roll_count(10)
///         .level("debug")
///         .output_file()
///         .output_console()
///         .build();
///
///     simple_log::new(old_config.clone())?;
///     let out = simple_log::get_log_conf()?;
///     assert_eq!(out, old_config);
///
///     debug!("test update_log_conf debug");
///     info!("test update_log_conf info");
///
///     let new_config = LogConfigBuilder::builder()
///         .path("./log/builder_log.log")
///         .size(2)
///         .roll_count(2)
///         .level("info")
///         .output_file()
///         .output_console()
///         .build();
///     simple_log::update_log_conf(new_config.clone())?;
///     let out = simple_log::get_log_conf()?;
///     assert_eq!(out, new_config);
///
///     debug!("test update_log_conf debug");//ignore
///     info!("test update_log_conf info");//print
///     Ok(())
/// }
///```
pub fn update_log_conf(log_config: LogConfig) -> SimpleResult<LogConfig> {
    let log_conf = LOG_CONF.get().unwrap();
    let mut guard = log_conf.lock().unwrap();
    let config = build_config(&log_config)?;
    guard.log_config = log_config;
    guard.handle.set_config(config);
    Ok(guard.log_config.clone())
}

/// update simple-log global config log level.
///
/// # Examples
///
/// ```rust
/// fn main() -> Result<(), String> {
///     use simple_log::{LogConfigBuilder, update_log_level, log_level};
///     let config = LogConfigBuilder::builder()
///         .path("./log/builder_log.log")
///         .size(1 * 64)
///        .roll_count(10)
///        .level("debug")
///        .output_file()
///        .output_console()
///        .build();
///     simple_log::new(config)?;
///
///     //update log level
///     let config = update_log_level(log_level::DEBUG)?;
///     assert_eq!("debug",config.get_level());
///     Ok(())
/// }
/// ```
///
pub fn update_log_level<S: Into<String>>(level: S) -> SimpleResult<LogConfig> {
    let log_conf = LOG_CONF.get().unwrap();
    let mut guard = log_conf.lock().unwrap();
    guard.log_config.level = level.into();
    let config = build_config(&guard.log_config)?;
    guard.handle.set_config(config);
    Ok(guard.log_config.clone())
}

/// Get simple-log global config [LogConfig]
///
/// ```rust
/// #[macro_use]
/// extern crate log;
///
/// use simple_log::LogConfigBuilder;
///
/// fn main() -> Result<(), String> {
///     let old_config = LogConfigBuilder::builder()
///         .path("./log/builder_log.log")
///         .size(1 * 100)
///         .roll_count(10)
///         .level("debug")
///         .output_file()
///         .output_console()
///         .build();
///
///     simple_log::new(old_config.clone())?;
///     let out = simple_log::get_log_conf()?;
///     assert_eq!(out, old_config);
///
///     debug!("test get_log_conf debug");
///     info!("test get_log_conf info");
///     Ok(())
/// }
/// ```
pub fn get_log_conf() -> SimpleResult<LogConfig> {
    let log_conf = LOG_CONF.get().unwrap();
    let config = log_conf.lock().unwrap().log_config.clone();
    Ok(config)
}

const SIMPLE_LOG_FILE: &str = "simple_log_file";
const SIMPLE_LOG_CONSOLE: &str = "simple_log_console";

use out_kind::deserialize_out_kind;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct LogConfig {
    path: String,
    level: String,
    size: u64,
    #[serde(deserialize_with = "deserialize_out_kind")]
    out_kind: Vec<OutKind>,
    roll_count: u32,
}

impl LogConfig {
    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn get_level(&self) -> &String {
        &self.level
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_out_kind(&self) -> &Vec<OutKind> {
        &self.out_kind
    }

    pub fn get_roll_count(&self) -> u32 {
        self.roll_count
    }
}

/// The [LogConfig] with builder wrapper.
pub struct LogConfigBuilder(LogConfig);

impl LogConfigBuilder {
    /// Construct a [LogConfig] by [`LogConfigBuilder::builder`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// fn run() {
    ///     use simple_log::{LogConfigBuilder, LogConfig};
    ///
    ///     let builder:LogConfigBuilder = LogConfigBuilder::builder();
    ///     let log_config:LogConfig = builder.build();
    ///     println!("{:?}",log_config);
    /// }
    /// ```
    ///
    pub fn builder() -> Self {
        LogConfigBuilder(LogConfig::default())
    }

    /// Receive file write path.
    ///
    /// simple-log output path when `OutKind` value is `File`.
    /// When `OutKind` value only is `console`,need ignore this method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// fn run() {
    ///     use simple_log::LogConfigBuilder;
    ///     use simple_log::LogConfig;
    ///
    ///     let builder:LogConfigBuilder = LogConfigBuilder::builder().path("/tmp/log/simple_log.log");
    ///     let config:LogConfig = builder.build();
    ///     println!("{:?}",config);
    /// }
    /// ```
    ///
    pub fn path<S: Into<String>>(mut self, path: S) -> LogConfigBuilder {
        self.0.path = path.into();
        self
    }

    ///
    pub fn level<S: Into<String>>(mut self, level: S) -> LogConfigBuilder {
        self.0.level = level.into();
        self
    }

    pub fn size(mut self, size: u64) -> LogConfigBuilder {
        self.0.size = size;
        self
    }

    pub fn output_file(mut self) -> LogConfigBuilder {
        self.0.out_kind.push(OutKind::File);
        self
    }

    /// Configuration [LogConfigBuilder] with log output with console.
    ///
    /// If your application build with `--release`.This method should not be used
    /// `output_file` method is recommended.
    /// This is usually used with `debug` or `test` mode.
    pub fn output_console(mut self) -> LogConfigBuilder {
        self.0.out_kind.push(OutKind::Console);
        self
    }

    pub fn roll_count(mut self, roll_count: u32) -> LogConfigBuilder {
        self.0.roll_count = roll_count;
        self
    }

    /// Constructs a new `LogConfig` .
    ///
    /// # Examples
    ///
    /// ```rust
    /// fn run() {
    ///     use simple_log::LogConfigBuilder;
    ///     let builder:LogConfigBuilder = LogConfigBuilder::builder();
    ///     let config = LogConfigBuilder::builder()
    ///         .path("./log/builder_log.log")
    ///         .size(1 * 100)
    ///        .roll_count(10)
    ///        .level("debug")
    ///        .output_file()
    ///        .output_console()
    ///        .build();
    ///     println!("{:?}",config);
    /// }
    /// ```
    pub fn build(self) -> LogConfig {
        self.0
    }
}

/// The [new] method provide init simple-log instance with config.
///
/// This method need pass [LogConfig] param. Your can use [LogConfigBuilder] `build` [LogConfig].
/// Also you can use [serde] with `Deserialize` init `LogConfig`.
///
/// # Examples
///
/// ```no_run
/// #[macro_use]
/// extern crate log;
///
/// use simple_log::LogConfigBuilder;
///
/// fn main() -> Result<(), String> {
///    let config = LogConfigBuilder::builder()
///            .path("./log/builder_log.log")
///            .size(1 * 100)
///            .roll_count(10)
///            .level("info")
///            .output_file()
///            .output_console()
///            .build();
///     simple_log::new(config)?;
///     debug!("test builder debug");
///     info!("test builder info");
///     Ok(())
/// }
/// ```
///
pub fn new(log_config: LogConfig) -> SimpleResult<()> {
    let mut log_config = log_config;
    init_default_log(&mut log_config);
    init_log_conf(log_config)?;
    Ok(())
}

/// This method can quick init simple-log with no configuration.
///
/// If your just want use in demo or test project. Your can use this method.
/// The [quick] method not add any params in method. It's so easy.
///
/// The [`LogConfig`] filed just used inner default value.
///
/// ```bash
///     path: ./tmp/simple_log.log //output file path
///     level: debug //log level
///     size: 10 //single log file size with unit:MB. 10MB eq:10*1024*1024
///     out_kind:[file,console] //Output to file and terminal at the same time
///     roll_count:10 //At the same time, it can save 10 files endwith .gz
///```
///
/// If you don't want use [quick] method.Also can use [new] method.
///
/// # Examples
///
/// ```rust
/// #[macro_use]
/// extern crate log;
///
/// fn main() -> Result<(), String> {
///     simple_log::quick()?;
///
///     debug!("test builder debug");
///     info!("test builder info");
///     Ok(())
/// }
/// ```
pub fn quick() -> SimpleResult<()> {
    let mut config = LogConfig::default();
    init_default_log(&mut config);
    init_log_conf(config)?;
    Ok(())
}

/// Provide init simple-log instance with stdout console on terminal.
///
/// Method receive log level one of [log_level] mod.
///
/// ```rust
/// #[macro_use]
/// extern crate log;
///
/// fn main() -> Result<(), String> {
///     simple_log::console("debug")?;
///
///     debug!("test console debug");
///     info!("test console info");
///     Ok(())
/// }
/// ```
pub fn console<S: Into<String>>(level: S) -> SimpleResult<()> {
    let config = LogConfig {
        path: "".to_string(),
        level: level.into(),
        size: 0,
        out_kind: vec![OutKind::Console],
        roll_count: 0,
    };
    init_log_conf(config)?;
    Ok(())
}

/// Provide init simple-log instance with write file.
///
/// The param `path` is either an absolute path or lacking a leading `/`, relative to the `cwd` of your [LogConfig].
///
/// The param `level` config log level with [log_level].
/// The param `size` config single file size(MB).
/// The param `roll_count` config single file size(MB).
///
/// The file extension of the pattern is `.gz`,the archive files will be gzip-compressed.
///
/// ```rust
/// #[macro_use]
/// extern crate log;
///
/// fn main() -> Result<(), String> {
///    simple_log::file("./log/file.log", "debug", 100, 10)?;
///
///    debug!("test file debug");
///    info!("test file info");
///    Ok(())
/// }
/// ```
pub fn file<S: Into<String>>(path: S, level: S, size: u64, roll_count: u32) -> SimpleResult<()> {
    let config = LogConfig {
        path: path.into(),
        level: level.into(),
        size,
        out_kind: vec![OutKind::File],
        roll_count,
    };
    init_log_conf(config)?;
    Ok(())
}

fn build_config(log: &LogConfig) -> SimpleResult<Config> {
    let mut config_builder = Config::builder();
    let mut root_builder = Root::builder();
    for kind in &log.out_kind {
        match kind {
            OutKind::File => {
                config_builder = config_builder
                    .appender(Appender::builder().build(SIMPLE_LOG_FILE, file_appender(log)?));
                root_builder = root_builder.appender(SIMPLE_LOG_FILE);
            }
            OutKind::Console => {
                let console = ConsoleAppender::builder()
                    .encoder(Box::new(encoder()))
                    .build();
                config_builder = config_builder
                    .appender(Appender::builder().build(SIMPLE_LOG_CONSOLE, Box::new(console)));
                root_builder = root_builder.appender(SIMPLE_LOG_CONSOLE);
            }
        }
    }

    let config = config_builder
        .build(root_builder.build(log_level::form_log_level(&log.level)))
        .map_err(|e| e.to_string())?;
    Ok(config)
}

/// check log config,and give default value
fn init_default_log(log: &mut LogConfig) {
    if log.path.trim().is_empty() {
        log.path = "./tmp/simple_log.log".to_string();
    }

    if log.size == 0 {
        log.size = 10 //1MB:1*1024*1024
    }

    if log.roll_count == 0 {
        log.roll_count = 10
    }

    if log.level.is_empty() {
        log.level = log_level::DEBUG.to_string()
    }

    if log.out_kind.is_empty() {
        log.out_kind
            .append(&mut vec![OutKind::Console, OutKind::File])
    }
}

fn encoder() -> PatternEncoder {
    PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S:%f)} [{l}] <{M}:{L}>:{m}\n")
}

fn file_appender(log: &LogConfig) -> SimpleResult<Box<RollingFileAppender>> {
    let roll = FixedWindowRoller::builder()
        .base(0)
        .build(format!("{}.{{}}.gz", log.path).as_str(), log.roll_count)
        .map_err(|e| e.to_string())?;

    let trigger = SizeTrigger::new(log.size * 1024 * 1024);

    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roll));

    let logfile = RollingFileAppender::builder()
        .encoder(Box::new(encoder()))
        .build(log.path.clone(), Box::new(policy))
        .map_err(|e| e.to_string())?;

    Ok(Box::new(logfile))
}

pub mod log_level {
    use log::LevelFilter;

    pub const TRACE: &str = "trace";
    pub const DEBUG: &str = "debug";
    pub const INFO: &str = "info";
    pub const WARN: &str = "warn";
    pub const ERROR: &str = "error";

    /// convert log level str to [LevelFilter].
    ///
    /// The default log level use [LevelFilter::Debug].
    ///
    /// # Examples
    ///
    /// ```rust
    /// fn run() {
    ///     use simple_log::log_level::form_log_level;
    ///     use log::LevelFilter;
    ///     let level = form_log_level("warn");
    ///     assert_eq!(level,LevelFilter::Warn);
    ///
    ///     let level = form_log_level("error");
    ///     assert_eq!(level,LevelFilter::Error);
    ///
    ///     let level = form_log_level("no");
    ///     assert_eq!(level,LevelFilter::Debug);
    /// }
    /// ```
    ///
    pub fn form_log_level(level: &str) -> LevelFilter {
        match level.to_lowercase().as_str() {
            TRACE => LevelFilter::Trace,
            DEBUG => LevelFilter::Debug,
            INFO => LevelFilter::Info,
            WARN => LevelFilter::Warn,
            ERROR => LevelFilter::Error,
            _ => LevelFilter::Debug,
        }
    }
}
