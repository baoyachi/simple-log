//! `cargo run --example new`
//!
//! With OutPut
//! ```bash
//! 2020-12-12 17:09:05:047820000 [DEBUG] <new:26>:test builder debug
//! 2020-12-12 17:09:05:048028000 [INFO] <new:27>:test builder info
//! ```

#[macro_use]
extern crate log;

use simple_log::LogConfigBuilder;

fn main() -> Result<(), String> {
    let config = LogConfigBuilder::builder()
        .path("./log/builder_log.log")
        .size(1 * 100)
        .roll_count(10)
        .level("debug")
        .output_file()
        .output_console()
        .build();

    simple_log::new(config)?;

    debug!("test builder debug");
    info!("test builder info");
    Ok(())
}
