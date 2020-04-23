use anyhow::Result;

use super::super::User;

// -------------------------
// Repository
// -------------------------

pub trait UserRepository {
    fn save(&self, user: User) -> Result<()>;
}

pub trait HaveUserRepository {
    type UserRepository: UserRepository;

    fn provide_user_repository(&self) -> &Self::UserRepository;
}
