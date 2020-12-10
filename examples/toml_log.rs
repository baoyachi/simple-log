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

    info!("info json simple_log");
    warn!("warn json simple_log");
    error!("error json simple_log");
}
