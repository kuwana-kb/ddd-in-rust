use std::str::FromStr;

use anyhow::Result;
use derive_more::Display;
use serde::{de, Serialize};

use crate::MyError;

#[derive(Clone, Debug, PartialEq, Eq, Display, Serialize)]
pub struct Name(String);

impl Name {
    pub fn new(s: &str) -> Result<Self> {
        if s.chars().count() < 3 || s.chars().count() > 20 {
            bail!(MyError::type_error("ユーザ名は3文字以上、20文字以下です"))
        }
        Ok(Name(s.to_string()))
    }
}

impl FromStr for Name {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl<'de> de::Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::new(&s).map_err(de::Error::custom)
    }
}
