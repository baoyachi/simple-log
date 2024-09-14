pub extern crate log;

#[macro_use]
pub mod macros;
#[cfg(feature = "log_inner")]
mod inner;
mod out_kind;
#[cfg(feature = "log_inner")]
pub use inner::*;

#[cfg(feature = "target")]
pub use simple_log_derive::*;

pub type SimpleResult<T> = Result<T, String>;
