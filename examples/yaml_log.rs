#[macro_use]
extern crate log;

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

    info!("info json simple_log");
    warn!("warn json simple_log");
    error!("error json simple_log");
}
