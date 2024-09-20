pub extern crate log;

#[macro_use]
pub mod macros;
#[cfg(feature = "log_inner")]
mod inner;
pub mod level;
mod out_kind;

#[cfg(feature = "log_inner")]
pub use inner::*;

pub use log::Level;
pub use log::LevelFilter;
#[cfg(feature = "target")]
pub use simple_log_derive::*;

pub type SimpleResult<T> = Result<T, String>;
