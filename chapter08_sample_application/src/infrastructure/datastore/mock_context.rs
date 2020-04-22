use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use common::MyError;

use crate::domain::{Name, User, UserId, UserRepository};

#[derive(Clone, Debug, Default)]
pub struct MockContext {
    db: Arc<Mutex<HashMap<UserId, User>>>,
}

impl UserRepository for MockContext {
    fn save(&self, user: User) -> Result<()> {
        let db = self.db.clone();
        let mut db = db
            .try_lock()
            .map_err(|_| MyError::internal_server_error("failed to try_lock db"))?;
        db.insert(user.id().clone(), user);
        Ok(())
    }

    fn find_by_name(&self, name: Name) -> Result<Option<User>> {
        let db = self.db.clone();
        let db = db
            .try_lock()
            .map_err(|_| MyError::internal_server_error("failed to try_lock db"))?;
        let target = db
            .values()
            .filter(|user| user.name().clone() == name)
            .cloned()
            .collect::<Vec<User>>();
        Ok(target.first().cloned())
    }

    fn find_by_id(&self, _id: UserId) -> Result<Option<User>> {
        unimplemented!()
    }

    fn delete(&self, _user: User) -> Result<()> {
        unimplemented!()
    }
}
