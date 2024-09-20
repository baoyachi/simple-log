use core::fmt;
use serde::de::SeqAccess;
use serde::{de, Deserialize, Deserializer, Serialize};

macro_rules! de_from {
    ($err:expr) => {
        KindSerde::deserialize($err).map_err(de::Error::custom)
    };
}

const KIND_FILE: &str = "file";
const KIND_CONSOLE: &str = "console";

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub enum OutKind {
    File,
    Console,
}

impl<'de> Deserialize<'de> for OutKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        let kind = de_from!(s)?;
        Ok(kind)
    }
}

const KIND_EXPECT: &str = "expect out_kind string or vec:'console','file' or ['console','file']";

impl<S: AsRef<str>> From<S> for OutKind {
    fn from(value: S) -> Self {
        KindSerde::deserialize(value.as_ref()).unwrap()
    }
}
pub struct KindSerde;

impl KindSerde {
    fn deserialize<S>(s: S) -> Result<OutKind, String>
    where
        S: Into<String>,
    {
        let s = s.into();
        match s.to_ascii_lowercase().as_str() {
            KIND_FILE => Ok(OutKind::File),
            KIND_CONSOLE => Ok(OutKind::Console),
            _ => Err(format!("Invalid state '{}',{}", s, KIND_EXPECT)),
        }
    }
}

impl<'de> de::Visitor<'de> for KindSerde {
    type Value = Vec<OutKind>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(KIND_EXPECT)
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let kind = de_from!(s)?;
        Ok(vec![kind])
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
    }
}

pub fn deserialize_out_kind<'de, D>(deserializer: D) -> Result<Vec<OutKind>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(KindSerde)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_out_kind_serde() {
        let json = r#"["Console", "File"]"#;
        let kind: Vec<OutKind> = serde_json::from_str(json).unwrap();
        assert_eq!(kind, vec![OutKind::Console, OutKind::File]);

        let json = r#"["console", "file"]"#;
        let kind: Vec<OutKind> = serde_json::from_str(json).unwrap();
        assert_eq!(kind, vec![OutKind::Console, OutKind::File]);

        let json = r#"["Console", "file"]"#;
        let kind: Vec<OutKind> = serde_json::from_str(json).unwrap();
        assert_eq!(kind, vec![OutKind::Console, OutKind::File]);

        let json = r#"["CONSOLE", "FILE"]"#;
        let kind: Vec<OutKind> = serde_json::from_str(json).unwrap();
        assert_eq!(kind, vec![OutKind::Console, OutKind::File]);
    }
}
