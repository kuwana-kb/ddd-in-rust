# DDD のパターンを Rust で表現する ~ Entity 編 ~

# 目次

# DDD とは

# DDD における Entity とは

Entity とは、一意なものを表現する概念です。
人(Human)で例えてみましょう。

ここに二人の人がいます。
二人の名前はどちらも「山田太郎」さんです。
では、この二人は全くの同一人物といえるでしょうか？
現実世界では、同姓同名がありえます。
つまり、二人は同じ名前でありながら、異なる一意な存在です。
Name という属性は同じでも、二人を区別できる必要が有ります。
これは、属性が同じであっても区別されることを示します。
また、二人を区別するための識別子が必要となることも示しています。

また、人は時を経て状態が変わります。
例えば、身長が伸びたり、体重が増えたりしますね。
これは、ライフサイクルを通じて Height や Weight といった属性が変化することと同義です。
つまり、Entity は可変であるといえるでしょう。

属性として登場した Name, Height, Weight は値が同じであれば同一とみなせます。
これらは Value Object として定義できるでしょう。
一方で、Human は Name, Height, Weight がすべて同一だとしても、同じとは限りません。
そして、ライフサイクルによって変化することがある。
これが Entity です。

# 実装パターンの紹介

## まずはシンプルに実装してみる

```rust
#![allow(dead_code)]
use common::MyError;

// Userモデルを表現したが、可変性と同一性を持たない状態
#[derive(Clone, Debug)]
pub struct User {
    name: String,
}

impl User {
    // ユーザー名は不変なため、後から変更することはできない
    pub fn new(name: String) -> Result<Self, MyError> {
        if name.chars().count() < 3 {
            bail!(MyError::type_error("ユーザー名は3文字以上です"))
        }
        Ok(Self { name })
    }
}
```

## 可変性を与える

```rust
#![allow(dead_code)]
use common::MyError;

// Userモデルに対して可変性を与えた
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: String) -> Result<Self, MyError> {
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
```

```rust
#![allow(dead_code)]
use common::MyError;

// Userモデルに対して可変性を与えた
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: Name) -> Result<Self, MyError> {
        Self {
            name: name,
        };
        Ok(user)
    }

    // Name 型を新たに Value Object として定義した
    // バリデーションのロジックは Name 型に移譲している
    pub fn change_name(&mut self, name: Name) -> Result<()> {
        self.name = name;
        Ok(())
    }
}

pub struct Name(String);

impl Name {
    pub fn new(s: String) -> Result<Self, MyError> {
        if name.chars().count() < 3 {
            bail!(MyError::type_error("ユーザー名は3文字以上です"))
        }
        Name(s)
    }
}
```

## 同一性を与える
