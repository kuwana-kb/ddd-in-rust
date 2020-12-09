#![allow(dead_code)]

use common::MyError;

// Userモデルを表現したが、可変性と同一性を持たない状態
#[derive(Clone, Debug)]
pub struct User {
    name: String,
}

impl User {
    // ユーザー名は不変なため、後から変更することはできない
    pub fn new(name: &str) -> Result<Self, MyError> {
        if name.chars().count() < 3 {
            return Err(MyError::type_error("ユーザー名は3文字以上です"));
        }
        Ok(Self {
            name: name.to_string(),
        })
    }
}
