//! `cargo run --example console`
//!
//! With OutPut
//! ```bash
//! 16:57:13.619320000 [DEBUG] <filter_module:25>:test console debug
//! 16:57:13.619481000 [INFO] <filter_module:26>:test console info
//! 16:57:13.619489000 [WARN] <filter_module:27>:test console warn
//! 16:57:13.619495000 [ERROR] <filter_module:28>:test console error
//! 16:57:13.619501000 [INFO] <filter_module::app:41>:init app
//! 16:57:13.619507000 [INFO] <filter_module::app::launch:46>:init launch
//! 16:57:13.619515000 [DEBUG] <filter_module::app::launch::parser:57>:parser log

//! ```

#[macro_use]
extern crate simple_log;

use log::LevelFilter;
use simple_log::LogConfig;

fn main() -> Result<(), String> {
    let config = r#"
    level = "debug,filter_module::app::ctrl=warn,filter_module::app::launch::conf=error"
    out_kind = "console"
    time_format = "%H:%M:%S.%f"
    "#;
    let conf: LogConfig = toml::from_str(config).unwrap();

    assert_eq!(
        conf,
        LogConfig {
            path: None,
            directory: None,
            level: (
                LevelFilter::Debug,
                vec![
                    ("filter_module::app::ctrl", LevelFilter::Warn).into(),
                    ("filter_module::app::launch::conf", LevelFilter::Error).into(),
                ]
            ),
            size: 0,
            out_kind: vec!["console".into()],
            roll_count: 0,
            time_format: Some("%H:%M:%S.%f".to_string()),
        }
    );

    simple_log::new(conf).unwrap(); //init log

    debug!("test console debug");
    info!("test console info");
    warn!("test console warn");
    error!("test console error");

    app::init_app();
    app::launch::init_launch();
    app::launch::conf::err_conf();
    app::launch::conf::debug_conf(); // this log filter
    app::launch::parser::err_parser();
    app::ctrl::init_ctrl(); // this log filter

    Ok(())
}

pub(crate) mod app {
    pub fn init_app() {
        info!("init app")
    }

    pub mod launch {
        pub fn init_launch() {
            info!("init launch")
        }

        pub mod conf {
            pub fn err_conf() {
                error!("conf log")
            }

            pub fn debug_conf() {
                debug!("conf log")
            }
        }

        pub mod parser {
            pub fn err_parser() {
                debug!("parser log")
            }
        }
    }

    pub mod ctrl {
        pub fn init_ctrl() {
            info!("init ctrl")
        }
    }
}
