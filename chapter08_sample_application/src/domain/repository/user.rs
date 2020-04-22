use anyhow::Result;

use super::super::{Name, User, UserId};

// -------------------------
// Repository
// -------------------------

pub trait UserRepository {
    fn find_by_id(&self, id: UserId) -> Result<Option<User>>;

    fn find_by_name(&self, name: Name) -> Result<Option<User>>;

    fn save(&self, user: User) -> Result<()>;

    fn delete(&self, user: User) -> Result<()>;
}

pub trait HaveUserRepository {
    type UserRepository: UserRepository;

    fn provide_user_repository(&self) -> &Self::UserRepository;
}
