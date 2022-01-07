//! `cargo run --example yaml_log`
//!
//! With Output
//! ```bash
//! 2020-12-12 17:17:03.937206000 [INFO] <yaml_log:30>:info yaml simple_log
//! 2020-12-12 17:17:03.937885000 [WARN] <yaml_log:31>:warn yaml simple_log
//! 2020-12-12 17:17:03.937970000 [ERROR] <yaml_log:32>:error yaml simple_log
//! ```

#[macro_use]
extern crate simple_log;

use simple_log::LogConfig;

fn main() {
    let config = r#"
    path: "./log/tmp.log"
    level: debug
    size: 10
    out_kind:
        - Console
        - File
    roll_count: 10

    "#;
    let config: LogConfig = serde_yaml::from_str(config).unwrap();

    simple_log::new(config).unwrap(); //init log

    info!("info yaml simple_log");
    warn!("warn yaml simple_log");
    error!("error yaml simple_log");
}
