use std::fmt;

use anyhow::Result;
use common::MyError;

use crate::{
    domain::{Name, User, UserFactory, UserRepository},
    infrastructure::PgPool,
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

// RDBにおけるUserFactoryの実装
// シーケンスを利用したファクトリとなっている
impl UserFactory for RDBContext {
    fn create(&self, name: Name) -> Result<User> {
        let mut client = self.pool.conn()?;

        let rows = client.query("SELECT seq = (NEXT VALUE FOR UserSeq)", &[])?;

        let row = rows.iter().next();
        let seq_id = match row {
            Some(row) => {
                let raw_seq_id: String = row.get(0);
                raw_seq_id
            }
            None => bail!(MyError::internal_server_error(
                "Failed to get sequential id."
            )),
        };
        let user = User::new(seq_id.parse()?, name);
        Ok(user)
    }
}

impl UserRepository for RDBContext {
    fn save(&self, _user: User) -> Result<()> {
        // 実装は省略する。chapter05_repositoryの実装を参照すること
        unimplemented!()
    }
}
