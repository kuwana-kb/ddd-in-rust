use std::str::FromStr;

use anyhow::Result;
use derive_more::Display;
use serde::{de, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, Display, Hash)]
pub struct UserId(String);

impl UserId {
    fn new(s: &str) -> Self {
        println!("user_id: {}", s);
        Self(s.to_string())
    }
}

impl FromStr for UserId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self::new(s))
    }
}

impl<'de> de::Deserialize<'de> for UserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::new(&s))
    }
}

impl Serialize for UserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}
