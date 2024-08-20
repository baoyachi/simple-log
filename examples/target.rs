//! `cargo run --example console`
//!
//! With OutPut
//! ```bash
//! 2024-08-20 15:43:09.309146000 [TRACE] [ctrl] <examples/target.rs:18>:test target trace
//! 2024-08-20 15:43:09.309336000 [TRACE] [parser] <examples/target.rs:19>:test target trace
//! 2024-08-20 15:43:09.309345000 [DEBUG] [target] <examples/target.rs:21>:test target debug
//! 2024-08-20 15:43:09.309352000 [DEBUG] [ctrl] <examples/target.rs:22>:test target debug
//! 2024-08-20 15:43:09.309358000 [DEBUG] [parser] <examples/target.rs:23>:test target debug
//! 2024-08-20 15:43:09.309364000 [INFO] [target] <examples/target.rs:25>:test target info
//! 2024-08-20 15:43:09.309371000 [INFO] [ctrl] <examples/target.rs:26>:test target info
//! 2024-08-20 15:43:09.309377000 [INFO] [parser] <examples/target.rs:27>:test target info
//! 2024-08-20 15:43:09.309383000 [WARN] [target] <examples/target.rs:29>:test target warn
//! 2024-08-20 15:43:09.309389000 [ERROR] [target] <examples/target.rs:30>:test target error
//! ```

#[macro_use]
extern crate simple_log;

fn main() -> Result<(), String> {
    simple_log::console("trace")?;

    simple_log::log_target!(ctrl);
    simple_log::log_target!(parser);

    trace_ctrl!("test target trace");
    trace_parser!("test target trace");

    debug!("test target debug");
    debug_ctrl!("test target debug");
    debug_parser!("test target debug");

    info!("test target info");
    info_ctrl!("test target info");
    info_parser!("test target info");

    warn!("test target warn");
    error!("test target error");

    Ok(())
}
