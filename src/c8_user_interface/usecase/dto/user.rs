use crate::c8_user_interface::domain::User;

// アプリケーションサービスのクライアントに対して公開するUserのDTO
#[derive(Clone, Debug)]
pub struct UserData {
    id: String,
    name: String,
}

// User型からDTOへの変換処理
impl From<User> for UserData {
    fn from(v: User) -> Self {
        Self {
            id: v.id().to_string(),
            name: v.name().to_string(),
        }
    }
}
