use anyhow::Result;
use derive_getters::Getters;
use derive_more::Display;
use std::str::FromStr;

use crate::MyError;

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
    // Name型が3文字以上を保証しているため、ここでチェックする必要はない
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
#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub struct UserId(String);

impl UserId {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// 名前
#[derive(Clone, Display, Debug)]
pub struct Name(String);

impl FromStr for Name {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.chars().count() < 3 {
            bail!(MyError::type_error("ユーザー名は3文字以上です"))
        }
        Ok(Self(s.to_string()))
    }
}

#[test]
fn test_user_eq() {
    let user_before = User::new(UserId::new("DummyId1"), "Hoge".parse().unwrap());
    let mut user_after = user_before.clone();
    user_after.change_username("Fuga".parse().unwrap());

    // beforeとafterで名前が異なる
    assert_eq!(user_before.name().to_string(), "Hoge".to_string()); // Ok
    assert_eq!(user_after.name().to_string(), "Fuga".to_string()); // Ok

    // PartialEq traitを実装したのでUserを比較可能
    assert_eq!(user_before, user_after); // Ok
}
