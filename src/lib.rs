#[macro_use]
extern crate serde_derive;

mod out_kind;

use crate::out_kind::OutKind;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use once_cell::sync::OnceCell;
use std::sync::Mutex;

type SimpleResult<T> = std::result::Result<T, String>;

static LOG_CONF: OnceCell<Mutex<LogConfig>> = OnceCell::new();

const SIMPLE_LOG_FILE: &str = "simple_log_file";
const SIMPLE_LOG_CONSOLE: &str = "simple_log_console";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LogConfig {
    path: String,
    level: String,
    size: u64,
    out_kind: Vec<OutKind>,
    roll_count: u32,
}

pub struct LogConfigBuilder(LogConfig);

impl LogConfigBuilder {
    /// Construct a [LogConfig] by [`LogConfigBuilder::builder`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use simple_log::{LogConfigBuilder, LogConfig};
    /// fn main() {
    ///     let builder:LogConfigBuilder = LogConfigBuilder::builder();
    /// }
    /// ```
    ///
    pub fn builder() -> Self {
        LogConfigBuilder(LogConfig::default())
    }

    /// Receive file write path.
    ///
    /// Simple-log output path when [OutKind] value is `File`.
    /// When [OutKind] value only is `console`,need ignore this method.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use simple_log::LogConfigBuilder;
    ///
    /// fn main() {
    ///  let builder:LogConfigBuilder = LogConfigBuilder::builder().path("/tmp/log/simple_log.log");
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

    pub fn output_console(mut self) -> LogConfigBuilder {
        self.0.out_kind.push(OutKind::Console);
        self
    }

    pub fn roll_count(mut self, roll_count: u32) -> LogConfigBuilder {
        self.0.roll_count = roll_count;
        self
    }

    pub fn build(self) -> LogConfig {
        self.0
    }
}

/// The [new] method provide init simple-log instance.
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
pub fn new(log: LogConfig) -> SimpleResult<()> {
    let mut log = log;
    init_default_log(&mut log);
    let config = build_config(&log)?;
    let handle = log4rs::init_config(config).map_err(|e| e.to_string())?;
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
/// ```no_run
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
    let config = build_config(&config)?;
    let handle = log4rs::init_config(config).map_err(|e| e.to_string())?;
    // LOG_CONF.get_or_init(||);
    Ok(())
}

pub fn console(level: String) -> SimpleResult<()> {
    let mut config = LogConfig::default();
    config.level = level;
    config.out_kind = vec![OutKind::Console];
    let config = build_config(&config)?;
    let handle = log4rs::init_config(config).map_err(|e| e.to_string())?;
    // LOG_CONF.get_or_init(||);
    Ok(())
}

pub fn file<S: Into<String>>(path: S, level: S, size: u64, roll_count: u32) -> SimpleResult<()> {
    let config = LogConfig {
        path: path.into(),
        level: level.into(),
        size,
        out_kind: vec![OutKind::File],
        roll_count,
    };
    let config = build_config(&config)?;
    let handle = log4rs::init_config(config).map_err(|e| e.to_string())?;
    // LOG_CONF.get_or_init(||);
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
        .build(root_builder.build(form_log_level(&log.level)))
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
        log.level = LOG_LEVEL_DEBUG.to_string()
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

pub const LOG_LEVEL_TRACE: &str = "trace";
pub const LOG_LEVEL_DEBUG: &str = "debug";
pub const LOG_LEVEL_INFO: &str = "info";
pub const LOG_LEVEL_WARN: &str = "warn";
pub const LOG_LEVEL_ERROR: &str = "error";

fn form_log_level(level: &String) -> LevelFilter {
    match level.to_lowercase().as_str() {
        LOG_LEVEL_TRACE => LevelFilter::Trace,
        LOG_LEVEL_DEBUG => LevelFilter::Debug,
        LOG_LEVEL_INFO => LevelFilter::Info,
        LOG_LEVEL_WARN => LevelFilter::Warn,
        LOG_LEVEL_ERROR => LevelFilter::Error,
        _ => LevelFilter::Debug,
    }
}
