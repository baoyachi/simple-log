use crate::InnerLevel;
use core::fmt;
use log::{Level, LevelFilter};
pub use parser::*;
use serde::de::DeserializeSeed;
use serde::{de, Deserializer};

pub(crate) mod parser {
    use crate::{InnerLevel, TargetLevel};
    use log::LevelFilter;
    use std::str::FromStr;
    use winnow::ascii::{alpha1, multispace0};
    use winnow::combinator::{opt, repeat};
    use winnow::token::take_while;
    use winnow::{Parser, Result as WResult};

    ///
    /// ```rust
    /// use log::LevelFilter;
    /// use simple_log::level::parse_level;
    /// let input = "off";
    /// assert_eq!(parse_level(input).unwrap(), (LevelFilter::Off, vec![]));
    ///
    /// let input = "debug";
    /// assert_eq!(parse_level(input).unwrap(), (LevelFilter::Debug, vec![]));
    ///
    /// let input = "info";
    /// assert_eq!(parse_level(input).unwrap(), (LevelFilter::Info, vec![]));
    ///
    /// let input = "warn";
    /// assert_eq!(parse_level(input).unwrap(), (LevelFilter::Warn, vec![]));
    ///
    /// let input = "error";
    /// assert_eq!(parse_level(input).unwrap(), (LevelFilter::Error, vec![]));
    ///
    /// let input = "off !!!";
    /// assert_eq!(parse_level(input).err().unwrap(),
    /// r###"Failed to parse level:
    /// off !!!
    ///    ^
    /// "###);
    ///
    /// let input = "warning";
    /// assert_eq!(parse_level(input).err().unwrap(),
    /// r#"Failed to parse level:
    /// warning
    /// ^
    /// attempted to convert a string that doesn't match an existing log level"#);
    ///
    ///
    /// let input = "info,";
    /// assert_eq!(parse_level(input).err().unwrap(),
    /// r#"Failed to parse level:
    /// info,
    ///     ^
    /// "#);
    ///
    /// let input = "error,app=off";
    /// assert_eq!(parse_level(input).unwrap(), (LevelFilter::Error, vec![("app", LevelFilter::Off).into()]));
    ///
    /// let input = "debug,app=error,";
    /// assert_eq!(parse_level(input).unwrap(), (LevelFilter::Debug, vec![("app", LevelFilter::Error).into()]));
    ///
    /// let input = "debug,app=error,filter_module::app::ctrl=error,app::launch::c123onf=info";
    /// assert_eq!(
    /// parse_level(input).unwrap(),
    ///  (LevelFilter::Debug, vec![
    ///   ("app", LevelFilter::Error).into(),
    ///   ("filter_module::app::ctrl", LevelFilter::Error).into(),
    ///   ("app::launch::c123onf", LevelFilter::Info).into(),
    ///  ]));
    ///
    ///```
    ///
    pub fn parse_level(input: &str) -> Result<InnerLevel, String> {
        match (
            level,
            opt((multispace0, ',', repeat(1.., target_level)))
                .map(|c| c.map(|(_, _, t)| t).unwrap_or_default()),
        )
            .parse(input)
        {
            Ok((level, targets)) => Ok((level, targets)),
            Err(err) => Err(format!("Failed to parse level:\n{}", err)),
        }
    }

    fn level(input: &mut &str) -> WResult<LevelFilter> {
        alpha1.try_map(LevelFilter::from_str).parse_next(input)
    }

    fn target_level(input: &mut &str) -> WResult<TargetLevel> {
        (multispace0, target_name, '=', level, multispace0, opt(','))
            .map(|(_, name, _, level, _, _)| (name, level).into())
            .parse_next(input)
    }

    fn target_name<'a>(input: &mut &'a str) -> WResult<&'a str> {
        take_while(1.., ('0'..='9', 'A'..='Z', 'a'..='z', ':', '_')).parse_next(input)
    }
}

#[allow(clippy::wrong_self_convention)]
pub trait LevelInto {
    fn into_level(&self) -> &str;
}

impl LevelInto for &str {
    fn into_level(&self) -> &str {
        self
    }
}

impl LevelInto for String {
    fn into_level(&self) -> &str {
        self.as_str()
    }
}

impl LevelInto for &String {
    fn into_level(&self) -> &str {
        self.as_str()
    }
}

impl LevelInto for LevelFilter {
    fn into_level(&self) -> &str {
        self.as_str()
    }
}

impl LevelInto for Level {
    fn into_level(&self) -> &str {
        self.as_str()
    }
}

macro_rules! de_from {
    ($err:expr) => {
        LevelSerde::deserialize($err).map_err(de::Error::custom)
    };
}

struct LevelSerde;

impl LevelSerde {
    fn deserialize<S>(s: S) -> Result<InnerLevel, String>
    where
        S: Into<String>,
    {
        let s = s.into();
        parse_level(&s)
    }
}

impl<'de> DeserializeSeed<'de> for LevelSerde {
    type Value = InnerLevel;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}

impl serde::de::Visitor<'_> for LevelSerde {
    type Value = InnerLevel;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("inner level")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        de_from!(s)
    }
}

pub fn deserialize_level<'de, D>(deserializer: D) -> Result<InnerLevel, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_any(LevelSerde)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_log_level1() {
        fn quick_log_level<S: LevelInto>(level: S) {
            level.into_level();
        }

        quick_log_level("debug");
        quick_log_level("debug".to_string());
        quick_log_level(LevelFilter::Debug);
        quick_log_level(Level::Debug);
    }
}
