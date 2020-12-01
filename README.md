# simple-log
A simple-log with local file or stdout write by Rust.


[![Chrono on crates.io][cratesio-image]][cratesio]
[![Chrono on docs.rs][docsrs-image]][docsrs]
[![dependency on depstatus][depstatus-image]][depstatus]

[cratesio-image]: https://img.shields.io/crates/v/simple-log.svg
[cratesio]: https://crates.io/crates/simple-log
[docsrs-image]: https://docs.rs/simple-log/badge.svg
[docsrs]: https://docs.rs/simple-log
[depstatus-image]: https://deps.rs/repo/github/baoyachi/simple-log/status.svg
[depstatus]:https://deps.rs/repo/github/baoyachi/simple-log

## Quick Use
```toml
[dependencies]
...
log = "0.4"
simple-log = "0.3"
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

## USAGE in project
```toml
[dependencies]
...
log = "0.4"
simple-log = "0.3"
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

