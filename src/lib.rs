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
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

#[cfg(feature = "target")]
pub use simple_log_derive::*;

pub type SimpleResult<T> = Result<T, String>;
pub(crate) type InnerLevel = (LevelFilter, Vec<TargetLevel>);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TargetLevel {
    name: String,
    level: LevelFilter,
}

impl<S> From<(S, LevelFilter)> for TargetLevel
where
    S: AsRef<str>,
{
    fn from(value: (S, LevelFilter)) -> Self {
        Self {
            name: value.0.as_ref().to_string(),
            level: value.1,
        }
    }
}

#[cfg(feature = "println")]
pub static SIMPLE_LOG_INSTANCE: OnceCell<()> = OnceCell::new();
