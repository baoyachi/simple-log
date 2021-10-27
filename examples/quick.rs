//! `cargo run --example quick`
//! With OutPut
//! ```bash
//! 2020-12-12 17:14:19.558272000 [DEBUG] <quick:14>:test quick debug
//! 2020-12-12 17:14:19.559048000 [INFO] <quick:15>:test quick info
//! ```

#[macro_use]
extern crate log;

fn main() {
    // quick_empty()

    // quick_level()

    quick_level_path()
}

fn quick_empty() {
    simple_log::quick!();

    debug!("test quick debug");
    info!("test quick info");
}

fn quick_level() {
    simple_log::quick!("info");

    debug!("test quick debug");
    info!("test quick info");
}

fn quick_level_path() {
    simple_log::quick!("info", "./log/tmp.log");

    debug!("test quick debug");
    info!("test quick info");
}
