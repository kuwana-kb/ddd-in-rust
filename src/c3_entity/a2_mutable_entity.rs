#![allow(dead_code)]

use anyhow::Result;

use crate::MyError;

// Userモデルに対して可変性を与えた
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: String) -> Result<Self> {
        let mut user = Self {
            name: Default::default(),
        };
        user.change_name(name)?;
        Ok(user)
    }
    // ふるまいを通じて属性を変更する
    // 変更ロジックはメソッド内に閉じ込めている
    // (個人的にはName型を定義して引数の時点で値を保証する方が好き)
    pub fn change_name(&mut self, name: String) -> Result<()> {
        if name.chars().count() < 3 {
            bail!(MyError::type_error("ユーザー名は3文字以上です"))
        }
        self.name = name;
        Ok(())
    }
}
