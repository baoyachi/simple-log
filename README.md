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
    

    simple_log::new();
    Ok(())
}
```

