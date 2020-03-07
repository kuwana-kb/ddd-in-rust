use std::str::FromStr;

use anyhow::Result;
use derive_more::Display;

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub struct MailAddress(String);

impl FromStr for MailAddress {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(MailAddress(s.to_string()))
    }
}
