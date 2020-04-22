use std::str::FromStr;

use anyhow::Result;
use common::MyError;
use derive_more::Display;
use serde::{de, Serialize, Serializer};
use ulid::Ulid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Display, Hash)]
pub struct UserId(Ulid);

impl UserId {
    fn new(s: &str) -> Result<Self> {
        println!("user_id: {}", s);
        Ok(UserId(
            Ulid::from_string(s).map_err(|_| MyError::type_error("IDに誤りがあります"))?,
        ))
    }
}

impl Default for UserId {
    fn default() -> Self {
        UserId(Ulid::new())
    }
}

impl FromStr for UserId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl<'de> de::Deserialize<'de> for UserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::new(&s).map_err(de::Error::custom)
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
