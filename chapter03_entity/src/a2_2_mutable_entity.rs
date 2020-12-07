#![allow(dead_code)]
use common::MyError;

// Userモデルに対して可変性を与えた
#[derive(Clone, Debug)]
pub struct User {
    name: Name,
}

impl User {
    pub fn new(name: Name) -> Self {
        Self { name: name }
    }

    // バリデーションのロジックは Name 型に移譲している
    pub fn change_name(&mut self, name: Name) {
        self.name = name;
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }
}

// Name 型を新たに Value Object として定義した
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
