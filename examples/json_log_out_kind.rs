//! `cargo run --example json_log_out_kind`
//! With OutPut
//! ```bash
//! 2020-12-12 17:12:55.665529000 [INFO] <json_log:29>:info json simple_log
//! 2020-12-12 17:12:55.666172000 [WARN] <json_log:30>:warn json simple_log
//! 2020-12-12 17:12:55.666256000 [ERROR] <json_log:31>:error json simple_log
//! ```
//!

#[macro_use]
extern crate simple_log;

use simple_log::LogConfig;

fn main() {
    let config = r#"
    {
        "path":"./log/tmp.log",
        "level":"debug",
        "size":10,
        "out_kind":"console",
        "roll_count":10,
        "time_format":"%H:%M:%S.%f"
    }"#;
    let log_config: LogConfig = serde_json::from_str(config).unwrap();

    simple_log::new(log_config).unwrap(); //init log

    info!("info json simple_log");
    warn!("warn json simple_log");
    error!("error json simple_log");
}
