#![allow(dead_code)]

use anyhow::Result;

use common::MyError;

// Userモデルに対して可変性を与えた
#[derive(Clone, Debug)]
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: &str) -> Result<Self> {
        let mut user = Self {
            name: Default::default(),
        };
        user.change_name(name)?;
        Ok(user)
    }

    // ふるまいを通じて属性を変更する
    // 変更ロジックはメソッド内に閉じ込めている
    // (個人的にはName型を定義して引数の時点で値を保証する方が好き)
    pub fn change_name(&mut self, name: &str) -> Result<()> {
        if name.chars().count() < 3 {
            bail!(MyError::type_error("ユーザー名は3文字以上です"))
        }
        self.name = name.to_string();
        Ok(())
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[test]
fn test_change_name_success() {
    let name_1 = "Hoge";
    let name_2 = "Fuga";
    let user_before_result = User::new(name_1);
    // new() が成功しているかをチェック
    assert!(user_before_result.is_ok());

    let user_before = user_before_result.unwrap();
    let mut user_after = user_before.clone();
    user_after.change_name(name_2).unwrap();

    // beforeとafterで名前が異なるかを検証
    assert_eq!(user_before.name(), name_1.to_string()); // Ok
    assert_eq!(user_after.name(), name_2.to_string()); // Ok
}
