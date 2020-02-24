use anyhow::Result;

use super::{IUserRepository, User, Name, UserId};

#[derive(Queryable)]
pub struct UserDataModel {
    id: String,
    name: String,
}


// TODO: contextはPoolでよい？
pub struct ORMUserRepository {
    ctx: PgPool,
}

impl ORMUserRepository {
    pub fn new(ctx: PgPool) {
        Self {
            ctx
        }
    }
}

impl IUserRepository for ORMUserRepository {
    fn save(&mut self, user: User) -> Result<()> {

    }

    fn find(&self, name: Name) -> Result<Option<User>> {
        unimplemented!()
    }
}
