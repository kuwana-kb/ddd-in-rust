use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use common::MyError;

use crate::domain::{Name, User, UserFactory, UserId, UserRepository};

#[derive(Clone, Debug, Default)]
pub struct MockContext {
    db: Arc<Mutex<HashMap<UserId, User>>>,
    counter: Arc<Mutex<i32>>,
}

// MockにおけるUserFactoryの実装
// MockContext内のカウンターを利用して採番する
impl UserFactory for MockContext {
    fn create(&self, name: Name) -> Result<User> {
        let counter = self.counter.clone();
        let mut counter = counter
            .try_lock()
            .map_err(|_| MyError::internal_server_error("failed to try_lock counter"))?;
        let id = *counter;
        let user = User::new(id.to_string().parse()?, name);

        *counter += 1;
        Ok(user)
    }
}

impl UserRepository for MockContext {
    fn save(&self, _user: User) -> Result<()> {
        // 実装は省略する。chapter05_repositoryの実装を参照すること
        unimplemented!()
    }
}
