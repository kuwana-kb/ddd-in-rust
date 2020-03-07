use std::str::FromStr;

use anyhow::Result;
use derive_more::Display;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Display, Hash)]
pub struct UserId(String);

impl Default for UserId {
    fn default() -> Self {
        UserId(Uuid::new_v4().to_string())
    }
}

impl FromStr for UserId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // ここでは省いているが、idの制約を満たすかチェックすること
        Ok(UserId(s.to_string()))
    }
}
