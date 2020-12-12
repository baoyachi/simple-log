//! `cargo run --example toml_log`
//!
//! With Output
//! ```bash
//! 2020-12-12 17:16:02:340877000 [INFO] <toml_log:37>:info toml simple_log
//! 2020-12-12 17:16:02:341504000 [WARN] <toml_log:38>:warn toml simple_log
//! 2020-12-12 17:16:02:341569000 [ERROR] <toml_log:39>:error toml simple_log
//! ```

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

use simple_log::LogConfig;

#[derive(Deserialize)]
struct LogConfigWrap {
    log_config: LogConfig,
}

fn main() {
    let config = r#"
    [log_config]
    path = "./log/tmp.log"
    level = "debug"
    size = 10
    out_kind = ["console","file"]
    roll_count = 10
    "#;
    let wrap: LogConfigWrap = toml::from_str(config).unwrap();

    simple_log::new(wrap.log_config).unwrap(); //init log

    info!("info toml simple_log");
    warn!("warn toml simple_log");
    error!("error toml simple_log");
}
