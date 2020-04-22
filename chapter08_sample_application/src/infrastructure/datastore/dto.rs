use std::convert::TryFrom;

use anyhow::Result;
use derive_getters::Getters;
use derive_new::new;

use crate::domain::User;

#[derive(Clone, Debug, Getters, new)]
pub struct UserDto {
    id: String,
    name: String,
    mail_address: String,
}

impl From<User> for UserDto {
    fn from(v: User) -> UserDto {
        Self {
            id: v.id().to_string(),
            name: v.name().to_string(),
            mail_address: v.mail_address().to_string(),
        }
    }
}

impl TryFrom<UserDto> for User {
    type Error = anyhow::Error;

    fn try_from(v: UserDto) -> Result<Self> {
        Ok(Self::rebuild(
            v.id().parse()?,
            v.name().parse()?,
            v.mail_address().parse()?,
        ))
    }
}
