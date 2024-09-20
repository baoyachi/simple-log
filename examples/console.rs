//! `cargo run --example console`
//!
//! With OutPut
//! ```bash
//! 2020-12-12 17:12:22.702511000 [DEBUG] <console:15>:test console debug
//! 2020-12-12 17:12:22.703261000 [INFO] <console:16>:test console info
//! ```

#[macro_use]
extern crate simple_log;

use log::Level;

fn main() -> Result<(), String> {
    simple_log::console(Level::Debug)?;

    debug!("test console debug");
    info!("test console info");
    warn!("test console warn");
    error!("test console error");

    Ok(())
}
