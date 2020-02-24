use std::collections::HashMap;

use anyhow::Result;
use derive_new::new;

use super::{IUserRepository, Name, User, UserId};

#[derive(Clone, new)]
pub struct InMemoryUserRepository {
    store: HashMap<UserId, User>,
}

impl IUserRepository for InMemoryUserRepository {
    // TODO: ここの実装をどうするか検討する
    // Mockは内部のHashMapにデータを保存する形にしたい
    // しかし、そうするとsaveメソッドのselfをmutにする必要がでてくる。これはinterfaceに影響を与えてしまうため要検討
    fn save(&mut self, user: User) -> Result<()> {
        println!("save: {:?}", user);
        self.store.insert(user.id().clone(), user);
        Ok(())
    }

    fn find(&self, name: Name) -> Result<Option<User>> {
        let target = self
            .store
            .values()
            // .filter(|user| user.name().clone() == name)
            .cloned()
            .collect::<Vec<User>>();
        println!("target: {:?}", target);
        Ok(target.first().cloned())
    }
}

#[test]
fn test_mock_repostitory() {
    use super::Program;

    let repo = InMemoryUserRepository::new(HashMap::new());
    let mut program = Program::new(repo);
    program.create_user("Hoge".parse().unwrap()).unwrap();
    let opt_user = program.repo().find("Hoge".parse().unwrap()).unwrap();
    assert!(opt_user.is_some());
}
