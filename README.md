# simple-log
A simple-log with local file or stdout write by Rust.


[![Crates.io](https://img.shields.io/crates/v/simple-log)](https://crates.io/crates/simple-log)
[![Crates.io](https://img.shields.io/crates/l/simple-log)](https://github.com/baoyachi/simple-log)
[![depstatus](https://deps.rs/repo/github/baoyachi/simple-log/status.svg)](https://deps.rs/repo/github/baoyachi/simple-log)
[![Crates.io](https://img.shields.io/crates/d/simple-log)](https://github.com/baoyachi/simple-log)

## simple-log format output    
```
2020-12-07 15:06:03.260570000 [INFO] <json_log:16>:info json simple_log
2020-12-07 15:06:03.262106000 [WARN] <json_log:17>:warn json simple_log
2020-12-07 15:06:03.262174000 [ERROR] <json_log:18>:error json simple_log
```


## Quick Use
```toml
[dependencies]
simple-log = "1.4.0"
```

```rust
#[macro_use]
extern crate simple_log;

fn main() {
    simple_log::quick!("info"); // also use empty args: simple_log::quick!();
    // simple_log::quick!(); //use debug log_level
    
    debug!("test quick debug");
    info!("test quick info");
}
```

## Usage in project
```toml
[dependencies]
simple-log = "1.4.0"
```
```rust
#[macro_use]
extern crate simple_log;

use simple_log::LogConfigBuilder;

fn main() -> Result<(), String> {
    let config = LogConfigBuilder::builder()
        .path("./log/builder_log.log")
        .size(1 * 100)
        .roll_count(10)
        .time_format("%Y-%m-%d %H:%M:%S.%f") //E.g:%H:%M:%S.%f
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

## Config with toml
```toml
[dependencies]
simple-log = "1.4.0"
toml = "0.5.7"
```

```rust
#[macro_use]
extern crate simple_log;

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
    out_kind = ["console","file"] # also configure only with file: out_kind = "file"  
    roll_count = 10
    time_format = "%H:%M:%S.%f"
    "#;
    let wrap: LogConfigWrap = toml::from_str(config).unwrap();

    simple_log::new(wrap.log_config).unwrap();//init log

    info!("info toml simple_log");
    warn!("warn toml simple_log");
    error!("error toml simple_log");
}
```

## Config with json

```toml
[dependencies]
simple-log = "1.4.0"
serde_json = "1"
```

```rust
#[macro_use]
extern crate simple_log;

use simple_log::LogConfig;

fn main() {
    let config = r#"
    {
        "path":"./log/tmp.log",
        "level":"debug",
        "size":10,
        "out_kind":["console","file"],
        "roll_count":10,
        "time_format":"%H:%M:%S.%f"
    }"#;
    let log_config: LogConfig = serde_json::from_str(config).unwrap();

    simple_log::new(log_config).unwrap();//init log

    info!("info json simple_log");
    warn!("warn json simple_log");
    error!("error json simple_log");
}
```

## examples
More than examples can see [examples](https://github.com/baoyachi/simple-log/tree/main/examples).
