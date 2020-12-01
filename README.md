# simple-log

A simple-log with local file or stdout.

## Quick Use
```toml
[dependencies]
simple-log = "0.2"
```

```rust
fn main() -> Result<(),String>{
    simple_log::quick();
    Ok(())
}
```

## USAGE in project
```toml
[dependencies]
simple-log = "0.2"
```
```rust
fn main() -> Result<(),String>{
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

