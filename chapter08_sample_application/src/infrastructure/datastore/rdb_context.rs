use std::convert::TryInto;
use std::fmt;

use anyhow::Result;

use crate::{
    domain::{Name, User, UserId, UserRepository},
    infrastructure::{PgPool, UserDto},
};

#[derive(Clone)]
pub struct RDBContext {
    pool: PgPool,
}

impl Default for RDBContext {
    fn default() -> Self {
        let pool = PgPool::new();
        Self { pool }
    }
}

// Debug traitの要求を満たすため仮実装
impl fmt::Debug for RDBContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "RDBContext debug")?;
        Ok(())
    }
}

impl UserRepository for RDBContext {
    fn save(&self, user: User) -> Result<()> {
        let mut client = self.pool.conn()?;

        let user: UserDto = user.into();
        let stmt = r#"
            INSERT INTO
                users
            VALUE
                ($1, $2, $3)
            ;
        "#;

        client.execute(stmt, &[user.id(), user.name(), user.mail_address()])?;
        Ok(())
    }

    fn find_by_name(&self, name: Name) -> Result<Option<User>> {
        let mut client = self.pool.conn()?;

        let stmt = r#"
            SELECT 
                * 
            FROM 
                users 
            WHERE
                name = $1
        "#;

        let row = client.query_one(stmt, &[&name.to_string()])?;

        let id = row.get("id");
        let name = row.get("name");
        let mail_address = row.get("mail_address");
        let dto = UserDto::new(id, name, mail_address);

        Ok(Some(dto.try_into()?))
    }

    fn find_by_id(&self, _id: UserId) -> Result<Option<User>> {
        unimplemented!()
    }

    fn delete(&self, _user: User) -> Result<()> {
        unimplemented!()
    }
}
