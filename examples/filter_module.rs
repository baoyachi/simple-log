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

use simple_log::LogConfig;

fn main() -> Result<(), String> {
    let config = r#"
    level = "debug"
    out_kind = "console"
    time_format = "%H:%M:%S.%f"
    filter_module = ["filter_module::app::ctrl","filter_module::app::launch::conf"]
    "#;
    let conf: LogConfig = toml::from_str(config).unwrap();

    simple_log::new(conf).unwrap(); //init log

    debug!("test console debug");
    info!("test console info");
    warn!("test console warn");
    error!("test console error");

    app::init_app();
    app::launch::init_launch();
    app::launch::conf::err_conf(); // this log filter
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
