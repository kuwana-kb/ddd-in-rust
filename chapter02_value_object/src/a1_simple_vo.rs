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
fn test_equality_of_vo() {
    let taro_tanaka_1 = FullName::new("taro", "tanaka");
    let taro_tanaka_2 = FullName::new("taro", "tanaka");
    let jiro_suzuki = FullName::new("jiro", "suzuki");

    // 値が同じVOの比較。一致する
    assert_eq!(taro_tanaka_1, taro_tanaka_2); // Ok

    // 値が異なるVOの比較。一致しない
    assert_ne!(taro_tanaka_1, jiro_suzuki); // Ok
}
