//! `cargo run --example println --features="println"`
//!
//! With OutPut
//! ```bash
//! 2025-03-28 15:28:51.346983000 [DEBUG] <println:4>:Hello, world!
//! 2025-03-28 15:28:51.347196000 [DEBUG] <println:5>:Hello, world!2
//! 2025-03-28 15:28:51.347214000 [DEBUG] <println:6>:Hello, world!3
//! 2025-03-28 15:28:51.347230000 [DEBUG] <println:7>:Hello, world!4
//! ```

use simple_log::println;

fn main() {
    println!("Hello, world!");
    println!("Hello, world!2");
    println!("Hello, world!3");
    println!("Hello, world!4");
}
