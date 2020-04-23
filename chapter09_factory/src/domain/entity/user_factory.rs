use super::User;
use anyhow::Result;

use crate::domain::Name;

pub trait UserFactory {
    fn create(&self, name: Name) -> Result<User>;
}

pub trait HaveUserFactory {
    type UserFactory: UserFactory;

    fn provide_user_factory(&self) -> &Self::UserFactory;
}
