use anyhow::Result;

use crate::domain::{HaveUserRepository, User, UserRepository};

// -------------------------
// Domain Service
// -------------------------

pub fn exists<T>(ctx: &T, user: &User) -> Result<bool>
where
    T: HaveUserRepository,
{
    let repo = ctx.provide_user_repository();
    let duplicated_user = repo.find_by_name(user.name().clone())?;
    match duplicated_user {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
