#![allow(dead_code)]

use derive_getters::Getters;

/// 氏名
// このケースではなるべくderiveを使い、コードの記述量を抑えている
// Getters, PartialEq, Eqによって`a1`で書いたgetterメソッドと比較の実装を省略している
// Getters: フィールドのgetterメソッドを生やすderiveマクロ
#[derive(Clone, Debug, Getters, PartialEq, Eq)]
pub struct FullName {
    first_name: String,
    last_name: String,
}

impl FullName {
    fn new(first_name: &str, last_name: &str) -> Self {
        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }
}

#[test]
fn test_full_name() {
    let name1 = FullName::new("taro", "tanaka");
    let name1_copied = name1.clone();
    let name2 = FullName::new("jiro", "suzuki");

    let result1 = name1 == name1_copied;
    let result2 = name1 == name2;

    println!("{}", result1);
    println!("{}", result2);
}
