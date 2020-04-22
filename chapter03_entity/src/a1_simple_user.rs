#![allow(dead_code)]

use anyhow::Result;

use common::MyError;

// Userモデルを表現したが、可変性と同一性を持たない状態
#[derive(Clone, Debug)]
pub struct User {
    name: String,
}

impl User {
    // ユーザー名は不変なため、後から変更することはできない
    pub fn new(name: String) -> Result<Self> {
        if name.chars().count() < 3 {
            bail!(MyError::type_error("ユーザー名は3文字以上です"))
        }
        Ok(Self { name })
    }
}
