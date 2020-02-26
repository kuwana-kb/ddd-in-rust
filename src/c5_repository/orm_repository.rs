use anyhow::Result;

use super::{IUserRepository, User, Name, UserId};

#[derive(Queryable)]
pub struct UserDataModel {
    id: String,
    name: String,
}

// MEMO: contextはConnectionをもたせる形が良いかも
pub struct ORMUserRepository {
    ctx: PgPool,
}

// TODO: Diesel使う形で書く
impl ORMUserRepository {
    pub fn new(ctx: PgPool) {
        Self {
            ctx
        }
    }
}

impl IUserRepository for ORMUserRepository {
    fn save(&mut self, user: User) -> Result<()> {
        unimplemented!()
    }

    fn find(&self, name: Name) -> Result<Option<User>> {
        unimplemented!()
    }
}
