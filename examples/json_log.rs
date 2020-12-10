#[macro_use]
extern crate log;

use simple_log::LogConfig;

fn main() {
    let config = r#"
    {
        "path":"./log/tmp.log",
        "level":"debug",
        "size":10,
        "out_kind":["console","file"],
        "roll_count":10
    }"#;
    let log_config: LogConfig = serde_json::from_str(config).unwrap();

    simple_log::new(log_config).unwrap(); //init log

    info!("info json simple_log");
    warn!("warn json simple_log");
    error!("error json simple_log");
}
