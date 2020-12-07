use serde::{de, Deserializer, Deserialize};

const KIND_FILE: &str = "file";
const KIND_CONSOLE: &str = "console";

#[derive(Debug, Serialize, PartialEq)]
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
        match s.as_str() {
            KIND_FILE => Ok(OutKind::File),
            KIND_CONSOLE => Ok(OutKind::Console),
            _ => { return Err(de::Error::custom(format!("Invalid state '{}'", s))); }
        }
    }
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
