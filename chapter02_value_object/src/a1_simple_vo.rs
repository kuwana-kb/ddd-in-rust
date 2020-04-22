#![allow(dead_code)]

/// 氏名
// このケースではなるべくderiveを使わずに直接実装を書いている
// そのためコードが長め
// また、フィールド(first_name, last_name)にはプリミティブな型を用いていて制約を設けていない
#[derive(Clone, Debug)]
pub struct FullName {
    first_name: String,
    last_name: String,
}

impl FullName {
    pub fn new(first_name: &str, last_name: &str) -> Self {
        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }
}

// trait PartialEqは半同値関係の性質を表す
// PartialEqを実装することで「==」演算子による比較が可能になる
impl PartialEq for FullName {
    fn eq(&self, other: &Self) -> bool {
        self.first_name() == other.first_name() && self.last_name() == other.last_name()
    }
}

#[test]
fn test_full_name() {
    let name1 = FullName::new("taro", "tanaka");
    let name1_copied = name1.clone();
    let name2 = FullName::new("jiro", "suzuki");

    // name1とそのコピーの比較
    assert_eq!(name1, name1_copied); // Ok
                                     // name1とnameの比較
    assert_ne!(name1, name2); // Ok
}
