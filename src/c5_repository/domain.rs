#![allow(dead_code)]

use std::str::FromStr;

use anyhow::Result;
use derive_getters::Getters;
use derive_more::Display;
use derive_new::new;

use crate::error::MyError;

// -------------------------
// Repository
// -------------------------

// Repositoryはふるまいを定義するためtraitで実装
pub trait IUserRepository: Clone {
    // 処理が失敗する可能性があるため、返り値はResult型で定義
    fn save(&self, user: User) -> Result<()>;

    // 処理が失敗する可能性がある&Userが存在しない可能性があるため、返り値はResult<Option<User>>型で定義
    fn find(&self, username: Name) -> Result<Option<User>>;

    // 永続化と関係がない&実装次第で動作が変わる危険性があるので、
    // 以下のようなメソッドはリポジトリとしては不適切
    // pub fn exists(exists: User) -> bool;
}

// -------------------------
// DomainService
// -------------------------

#[derive(Clone, Debug, new, Getters)]
pub struct Program<Repo: IUserRepository> {
    repo: Repo,
}

impl<Repo> Program<Repo>
where
    Repo: IUserRepository,
{
    pub fn create_user(&mut self, username: Name) -> Result<()> {
        let user = User::new(username);

        let user_service = UserService::new(self.repo());
        if user_service.exists(user.clone())? {
            bail!(MyError::internal_server_error(
                "対象のユーザ名は既に存在しています。"
            ))
        }
        self.repo.save(user)
    }
}

#[derive(Clone, Debug, Getters)]
pub struct UserService<Repo: IUserRepository> {
    repo: Repo,
}

// TODO: 内部処理の実装
impl<Repo> UserService<Repo>
where
    Repo: IUserRepository,
{
    pub fn new(repo: &Repo) -> Self {
        Self { repo: repo.clone() }
    }

    pub fn exists(&self, user: User) -> Result<bool> {
        let result = self.repo.find(user.name().clone())?;
        match result {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}

// -------------------------
// Entity & Value Object
// -------------------------

#[derive(Clone, Debug, Getters)]
pub struct User {
    id: UserId,
    name: Name,
}

impl User {
    pub fn new(name: Name) -> Self {
        Self {
            id: UserId::default(),
            name,
        }
    }

    pub fn rebuild(id: UserId, name: Name) -> Self {
        Self { id, name }
    }
}

#[derive(Clone, Debug, Display, Hash, PartialEq, Eq)]
pub struct UserId(String);

impl Default for UserId {
    fn default() -> Self {
        // TODO: uuid等で自動生成する
        UserId("DummyId".to_string())
    }
}

impl FromStr for UserId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // 生成時の制約がある場合、それを満たすようにすること
        Ok(UserId(s.to_string()))
    }
}

#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub struct Name(String);

impl FromStr for Name {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Name(s.to_string()))
    }
}
