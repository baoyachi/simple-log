//! `cargo run --example file`
//!
//! With OutPut
//! ```bash
//! 2020-12-12 17:09:05.047820000 [DEBUG] <file:16>:test file debug
//! 2020-12-12 17:09:05.048028000 [INFO] <file:17>:test file info
//! ```

#[macro_use]
extern crate simple_log;

fn main() -> Result<(), String> {
    simple_log::file("./log/file.log", "debug", 100, 10)?;

    debug!("test file debug");
    info!("test file info");
    Ok(())
}
