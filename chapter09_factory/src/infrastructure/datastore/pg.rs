use std::ops::{Deref, DerefMut};
use std::time::Duration;

use anyhow::Result;
use postgres::{Config, NoTls};
use r2d2_postgres::{
    r2d2::{Pool, PooledConnection},
    PostgresConnectionManager,
};

pub struct PgConn(PooledConnection<PostgresConnectionManager<NoTls>>);

impl Deref for PgConn {
    type Target = PooledConnection<PostgresConnectionManager<NoTls>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PgConn {
    fn deref_mut(&mut self) -> &mut PooledConnection<PostgresConnectionManager<NoTls>> {
        &mut self.0
    }
}

#[derive(Clone)]
pub struct PgPool(Pool<PostgresConnectionManager<NoTls>>);

impl PgPool {
    pub fn new() -> Self {
        // ハードコードしているが、envy等で環境変数から取得する形にもできる
        let config = Config::new()
            .user("postgres")
            .password("password")
            .host("localhost")
            .port(5432)
            .dbname("hoge")
            .to_owned();

        let manager = PostgresConnectionManager::<NoTls>::new(config, NoTls);
        let pool = Pool::builder()
            .connection_timeout(Duration::from_secs(10))
            .build_unchecked(manager);

        PgPool(pool)
    }

    pub fn conn(&self) -> Result<PgConn> {
        let pool = self.0.clone();
        let conn = pool.get()?;
        Ok(PgConn(conn))
    }
}
