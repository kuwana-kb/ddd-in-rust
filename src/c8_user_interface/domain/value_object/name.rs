use std::str::FromStr;

use anyhow::Result;
use derive_more::Display;

use crate::MyError;

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub struct Name(String);

impl FromStr for Name {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.chars().count() < 3 || s.chars().count() > 20 {
            bail!(MyError::type_error("ユーザ名は3文字以上、20文字以下です"))
        }
        Ok(Name(s.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub struct MailAddress(String);
