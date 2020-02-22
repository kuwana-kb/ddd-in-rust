use std::convert::{TryFrom, TryInto};

use anyhow::Result;

use crate::c3_entity::{Name, User, UserId};

// Repositoryはふるまいを定義するためtraitで実装
pub trait IUserRepository {
    // 失敗する可能性があるため返り値はResult型
    fn save(user: User) -> Result<()>;

    // 失敗する可能性がある&複数件に対応するため返り値はResult<Vec<User>>型
    fn find(username: Name) -> Result<Vec<User>>;

    // 永続化と関係がない&実装次第で動作が変わる危険性があるので、
    // 以下のようなメソッドはリポジトリとしては不適切
    // pub fn exists(exists: User) -> bool;
}

use postgres::{Client, NoTls};

pub struct App {}

const CONNECTION_STRING: &str = "host=localhost user=postgres";

impl IUserRepository for App {
    fn save(user: User) -> Result<()> {
        // サンプルとしては、DBを操作するための簡易的な実装に留める
        // ConnectionPoolをアプリケーションを表すstructに対して持たせて引き回す方がよいはず
        let mut client = Client::connect(CONNECTION_STRING, NoTls)?;

        let id = user.id().to_string();
        let name = user.name().to_string();
        client.execute(
            "INSERT INTO users (id, name) VALUES ($1, $2)",
            &[&id, &name],
        )?;

        Ok(())
    }

    fn find(username: Name) -> Result<Vec<User>> {
        // サンプルとしては、DBを操作するための簡易的な実装に留める
        // ConnectionPoolをアプリケーションを表すstructに対して持たせて引き回す方がよいはず
        let mut client = Client::connect(CONNECTION_STRING, NoTls)?;

        let name = username.to_string();

        let mut dtos = Vec::new();
        for row in client.query("SELECT id, name FROM users WHERE name = $1", &[&name])? {
            let user_dto = UserDto {
                id: row.get(0),
                username: row.get(1),
            };
            // 取得した結果をプリミティブな型のUserDtoとして保存する
            dtos.push(user_dto);
        }
        // UserDto -> Userの型変換
        let users = dtos
            .iter()
            .map(|user_dto| user_dto.try_into())
            .collect::<Result<Vec<User>>>()?;

        Ok(users)
    }
}

// users tableの取得結果を格納するオブジェクト
pub struct UserDto {
    id: String,
    username: String,
}

// UserDto型からUser型への変換処理
//
// 失敗の可能性を考慮するならTryFromで実装
// データストアに保存された時点で値が制約に基づいているという前提で、
// 失敗の可能性はないと判断してFromで実装することもできるはず
// ただしその場合、unwrap()を使って失敗するとpanicする点に注意
impl TryFrom<&UserDto> for User {
    type Error = anyhow::Error;

    fn try_from(v: &UserDto) -> Result<Self> {
        let id = UserId::new(v.id.as_str());
        let name = v.username.parse()?;
        Ok(User::new(id, name))
    }
}
