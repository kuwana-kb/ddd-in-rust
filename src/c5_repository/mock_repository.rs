use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use derive_new::new;

use super::{IUserRepository, Name, User, UserId};
use crate::MyError;

#[derive(Clone, new)]
pub struct InMemoryUserRepository {
    store: Arc<Mutex<HashMap<UserId, User>>>,
}

impl IUserRepository for InMemoryUserRepository {
    fn save(&self, user: User) -> Result<()> {
        let store = self.store.clone();
        let mut store = store
            .try_lock()
            .map_err(|_| MyError::internal_server_error("failed to try_lock store"))?;
        store.insert(user.id().clone(), user);
        Ok(())
    }

    fn find(&self, name: Name) -> Result<Option<User>> {
        let store = self.store.clone();
        let store = store
            .try_lock()
            .map_err(|_| MyError::internal_server_error("failed to try_lock store"))?;
        let target = store
            .values()
            .filter(|user| user.name().clone() == name)
            .cloned()
            .collect::<Vec<User>>();
        Ok(target.first().cloned())
    }
}

#[test]
fn test_mock_repostitory() {
    use super::Program;

    let repo = InMemoryUserRepository::new(Arc::new(Mutex::new(HashMap::new())));
    let mut program = Program::new(repo);
    program.create_user("Hoge".parse().unwrap()).unwrap();
    let opt_user = program.repo().find("Hoge".parse().unwrap()).unwrap();
    assert!(opt_user.is_some());
}
