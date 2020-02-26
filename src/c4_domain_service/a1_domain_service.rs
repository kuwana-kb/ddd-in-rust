#![allow(dead_code)]

use derive_new::new;

use crate::c3_entity::User;

/// UserServiceはUserが持つと不自然なふるまいを受け持つ
#[derive(Debug, new)]
pub struct UserService {}

impl UserService {
    // ユーザー名が重複するかを確認する
    pub fn exists(&self, _user: User) -> bool {
        // User.usernameの重複をチェックするような実装
        // Infra周りの実装は冗長でなのでRepositoryで扱いたい
        unimplemented!()
    }

    // 「不自然なふるまい」は実装者の考え次第では、あらゆるふるまいに対して適用できてしまう
    // あらゆるふるまいがDomain Service上で表現されると、Domainオブジェクトの表現が削がれて、ドメインモデル貧血症を招く
    //
    // 例えば、以下のふるまいはUser上に実装する方がよい
    // pub fn change_name(&mut user: User, name: Name) {
    //     user.name = name;
    // }
}

#[test]
fn check_user() {
    use crate::c3_entity::UserId;

    // UserServiceの使い方を表すサンプル
    let user_service = UserService::new();

    let user = User::new(UserId::new("id"), "Hoge".parse().unwrap());
    // let duplicate_check_result = user_service.exists(user); // 実装がないためpanicする
}
