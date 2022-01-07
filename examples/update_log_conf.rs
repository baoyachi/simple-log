//! `cargo run --example update_log_conf`
//!
//! With OutPut
//! ```bash
//! 2020-12-12 17:51:34.624199000 [DEBUG] <get_log_conf:28>:test update_log_conf debug
//! 2020-12-12 17:51:34.624294000 [INFO] <get_log_conf:29>:test update_log_conf info
//! 2020-12-12 17:54:15.053281000 [INFO] <update_log_conf:41>:test update_log_conf info
//! ```

#[macro_use]
extern crate simple_log;

use simple_log::LogConfigBuilder;

fn main() -> Result<(), String> {
    let old_config = LogConfigBuilder::builder()
        .path("./log/builder_log.log")
        .size(1 * 100)
        .roll_count(10)
        .level("debug")
        .output_file()
        .output_console()
        .build();

    simple_log::new(old_config.clone())?;
    let out = simple_log::get_log_conf()?;
    assert_eq!(out, old_config);

    debug!("test update_log_conf debug");
    info!("test update_log_conf info");

    let new_config = LogConfigBuilder::builder()
        .path("./log/builder_log.log")
        .size(2)
        .roll_count(2)
        .level("info")
        .output_file()
        .output_console()
        .build();
    simple_log::update_log_conf(new_config.clone())?;
    let out = simple_log::get_log_conf()?;
    assert_eq!(out, new_config);

    debug!("test update_log_conf debug");
    info!("test update_log_conf info");
    Ok(())
}
