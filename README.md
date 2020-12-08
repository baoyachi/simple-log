# simple-log
A simple-log with local file or stdout write by Rust.


[![Crates.io](https://img.shields.io/crates/v/simple-log)](https://crates.io/crates/simple-log)
[![Crates.io](https://img.shields.io/crates/l/simple-log)](https://github.com/baoyachi/simple-log)
[![depstatus](https://deps.rs/repo/github/baoyachi/simple-log/status.svg)](https://deps.rs/repo/github/baoyachi/simple-log)
[![Crates.io](https://img.shields.io/crates/d/simple-log)](https://github.com/baoyachi/simple-log)

## Quick Use
```toml
[dependencies]
log = "0.4"
simple-log = "1.0.0"
```

```rust
#[macro_use]
extern crate log;

fn main() -> Result<(), String> {
    simple_log::quick()?;

    debug!("test builder debug");
    info!("test builder info");
    Ok(())
}
```

## Usage in project
```toml
[dependencies]
log = "0.4"
simple-log = "1.0.0"
```
```rust
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
```

## simple-log format output   
```
2020-12-07 15:06:03:260570000 [INFO] <json_log:16>:info json simple_log
2020-12-07 15:06:03:262106000 [WARN] <json_log:17>:warn json simple_log
2020-12-07 15:06:03:262174000 [ERROR] <json_log:18>:error json simple_log
```

