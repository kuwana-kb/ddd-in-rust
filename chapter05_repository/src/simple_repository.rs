use std::convert::{TryFrom, TryInto};

use anyhow::Result;
use derive_new::new;
use postgres::{Client, NoTls};

use super::{IUserRepository, Name, User};

// -------------------------
// Infrastucture層
// -------------------------

#[derive(Clone, Debug, new)]
pub struct UserRepository {}

// const CONNECTION_STRING: &str = "host=localhost user=postgres";
const CONNECTION_STRING: &str = "postgres://postgres:password@localhost:5432/users";

impl IUserRepository for UserRepository {
    fn save(&self, user: User) -> Result<()> {
        // サンプルとしては、DBを操作するための簡易的な実装に留める
        // ConnectionPoolをアプリケーションを表すstructに対して持たせて引き回す方がよいはず
        let mut client = Client::connect(CONNECTION_STRING, NoTls)?;

        let id = user.id().to_string();
        let name = user.name().to_string();
        client.execute(
            // r#"INSERT INTO users (id, name) VALUES ($1, $2)"#,
            r#"
            INSERT INTO users (id, name)
            VALUES ($1, $2)
            ON CONFLICT(id)
            DO UPDATE SET name = $2;
            "#,
            &[&id, &name],
        )?;

        Ok(())
    }

    fn find(&self, name: Name) -> Result<Option<User>> {
        // サンプルとしては、DBを操作するための簡易的な実装に留める
        // ConnectionPoolをアプリケーションを表すstructに対して持たせて引き回す方がよいはず
        let mut client = Client::connect(CONNECTION_STRING, NoTls)?;

        let name = name.to_string();
        let rows = client.query("SELECT id, name FROM users WHERE name = $1", &[&name])?;

        // ここでは取得結果が0件また1件という前提で処理している
        let row = rows.iter().next();
        match row {
            Some(row) => {
                // 取得した結果をプリミティブな型のUserDtoに格納する
                let user_dto = UserDto {
                    id: row.get(0),
                    name: row.get(1),
                };
                // TryIntoでUser型に変換してから返す
                Ok(Some(user_dto.try_into()?))
            }
            None => Ok(None),
        }
    }
}

// users tableの取得結果を格納するオブジェクト
pub struct UserDto {
    id: String,
    name: String,
}

// UserDto型からUser型への変換処理
//
// 失敗の可能性を考慮するならTryFromで実装
// データストアに保存された時点で値が制約に基づいているという前提で、
// 失敗の可能性はないと判断してFromで実装することもできるはず
// ただしその場合、unwrap()を使って失敗するとpanicする点に注意
impl TryFrom<UserDto> for User {
    type Error = anyhow::Error;

    fn try_from(v: UserDto) -> Result<Self> {
        let id = v.id.parse()?;
        let name = v.name.parse()?;
        Ok(User::rebuild(id, name))
    }
}

// To run this test:
// cargo test -- --ignored
#[test]
#[ignore]
fn repository_example() {
    use super::Program;

    let repo = UserRepository::new();
    let mut program = Program::new(repo);
    program.create_user("Taro".parse().unwrap()).unwrap(); // DBインスタンスをたてていないと失敗します。
}
