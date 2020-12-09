#![allow(dead_code)]

use anyhow::Result;
use common::MyError;
use derive_getters::Getters;

/// Userモデルに対して可変性と同一性を与えた
#[derive(Clone, Debug, Getters)]
pub struct User {
    id: UserId,
    name: Name,
}

impl User {
    pub fn new(id: UserId, name: Name) -> Self {
        Self { id, name }
    }

    // nameフィールドは可変性を持つ
    pub fn change_username(&mut self, name: Name) {
        self.name = name;
    }
}

// Userは識別子による比較が可能
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// trait Eqは同値関係の性質を表す
impl Eq for User {}

/// ユーザーID
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserId(String);

impl UserId {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// 名前
#[derive(Clone, Debug)]
pub struct Name(String);

impl Name {
    pub fn new(s: &str) -> Result<Self, anyhow::Error> {
        if s.chars().count() < 3 {
            bail!(MyError::type_error("ユーザー名は3文字以上です"))
        }
        Ok(Name(s.to_string()))
    }
}

#[test]
fn test_user_eq() {
    let user_before = User::new(UserId::new("DummyId1"), Name::new("Hoge").unwrap());
    let mut user_after = user_before.clone();
    user_after.change_username(Name::new("Fuga").unwrap());

    // User の属性を変更しても、同一性は同じまま
    // PartialEq trait を実装したので User を比較可能
    assert_eq!(user_before, user_after); // Ok
}
