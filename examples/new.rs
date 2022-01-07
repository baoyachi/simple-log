//! `cargo run --example new`
//!
//! With OutPut
//! ```bash
//! 2020-12-12 17:09:05.047820000 [DEBUG] <new:26>:test new debug
//! 2020-12-12 17:09:05.048028000 [INFO] <new:27>:test new info
//! ```

#[macro_use]
extern crate simple_log;

use simple_log::LogConfigBuilder;

fn main() -> Result<(), String> {
    let config = LogConfigBuilder::builder()
        .path("./log/builder_log.log")
        .size(100)
        .roll_count(10)
        .level("debug")
        .time_format("%Y-%m-%d %H:%M:%S.%f")
        .output_file()
        .output_console()
        .build();

    simple_log::new(config)?;

    debug!("test new debug");
    info!("test new info");
    Ok(())
}
