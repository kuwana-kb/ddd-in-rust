#![allow(dead_code)]
use std::str::FromStr;

use anyhow::Result;
use derive_getters::Getters;
use derive_more::Display;
use uuid::Uuid;

use crate::MyError;

// -------------------------
// Entity & Value Object
// -------------------------

#[derive(Clone, Debug, Getters)]
pub struct User {
    id: UserId,
    name: Name,
    mail_address: MailAddress,
}

impl User {
    // はじめてインスタンスを生成する際に利用する
    pub fn new(name: Name, mail_address: MailAddress) -> Self {
        Self {
            id: UserId::default(),
            name,
            mail_address,
        }
    }

    // インスタンスを再構成する際に利用する
    pub fn rebuild(id: UserId, name: Name, mail_address: MailAddress) -> Self {
        Self {
            id,
            name,
            mail_address,
        }
    }

    pub fn change_name(&mut self, name: Name) {
        self.name = name;
    }

    pub fn change_mail_address(&mut self, mail_address: MailAddress) {
        self.mail_address = mail_address;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Display, Hash)]
pub struct UserId(String);

impl Default for UserId {
    fn default() -> Self {
        UserId(Uuid::new_v4().to_string())
    }
}

impl FromStr for UserId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // ここでは省いているが、idの制約を満たすかチェックすること
        Ok(UserId(s.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub struct Name(String);

impl FromStr for Name {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.chars().count() < 3 || s.chars().count() > 20 {
            bail!(MyError::type_error("ユーザ名は3文字以上、20文字以下です"))
        }
        Ok(Name(s.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub struct MailAddress(String);

// -------------------------
// Domain Service
// -------------------------

pub fn exists<T>(ctx: &T, user: &User) -> Result<bool>
where
    T: HaveUserRepository,
{
    let repo = ctx.provide_user_repository();
    let duplicated_user = repo.find_by_name(user.clone().name)?;
    match duplicated_user {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

// -------------------------
// Repository
// -------------------------

pub trait UserRepository {
    fn find_by_id(&self, id: UserId) -> Result<Option<User>>;

    fn find_by_name(&self, name: Name) -> Result<Option<User>>;

    fn save(&self, user: User) -> Result<()>;

    fn delete(&self, user: User) -> Result<()>;
}

pub trait HaveUserRepository {
    type UserRepository: UserRepository;

    fn provide_user_repository(&self) -> &Self::UserRepository;
}
