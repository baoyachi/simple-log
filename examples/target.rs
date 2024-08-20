//! `cargo run --example console`
//!
//! With OutPut
//! ```bash
//! 2024-08-20 16:11:16.380183000 [TRACE] [ctrl] <examples/target.rs:26>:test target trace
//! 2024-08-20 16:11:16.380376000 [TRACE] [parser] <examples/target.rs:27>:test target trace
//! 2024-08-20 16:11:16.380388000 [DEBUG] [target] <examples/target.rs:29>:test target debug
//! 2024-08-20 16:11:16.380396000 [DEBUG] [ctrl] <examples/target.rs:30>:test target debug
//! 2024-08-20 16:11:16.380402000 [DEBUG] [parser] <examples/target.rs:31>:test target debug
//! 2024-08-20 16:11:16.380409000 [INFO] [target] <examples/target.rs:33>:test target info
//! 2024-08-20 16:11:16.380416000 [INFO] [ctrl] <examples/target.rs:34>:test target info
//! 2024-08-20 16:11:16.380422000 [INFO] [parser] <examples/target.rs:35>:test target info
//! 2024-08-20 16:11:16.380428000 [WARN] [target] <examples/target.rs:37>:test target warn
//! 2024-08-20 16:11:16.380434000 [ERROR] [target] <examples/target.rs:38>:test target error
//! 2024-08-20 16:11:16.380441000 [INFO] [conf] <examples/target.rs:40>:test target conf
//! 2024-08-20 16:11:16.380447000 [INFO] [bench] <examples/target.rs:41>:test target bench
//! ```

#[macro_use]
extern crate simple_log;

fn main() -> Result<(), String> {
    simple_log::console("trace")?;

    simple_log::log_target!(ctrl);
    simple_log::log_target!(parser, launch, conf, bench);

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

    info_conf!("test target conf");
    info_bench!("test target bench");

    Ok(())
}
