use std::str::FromStr;

use anyhow::Result;
use derive_getters::Getters;
use derive_more::Display;
use derive_new::new;
use uuid::Uuid;

use super::{IUserRepository, MailAddress, Name, User, UserId, UserService};
use crate::MyError;

#[derive(Clone, Debug, Getters)]
pub struct UserApplicationService<Repo>
where
    Repo: IUserRepository,
{
    repo: Repo,
    user_service: UserService<Repo>,
}

// MEMO: ドメインオブジェクトを公開しない前提ならば、
// アプリケーションサービスのもつメソッドの引数は全てプリミティブか型の方がよいかも？
impl<Repo> UserApplicationService<Repo>
where
    Repo: IUserRepository,
{
    pub fn new(repo: Repo, user_service: UserService<Repo>) -> Self {
        Self { repo, user_service }
    }

    pub fn register(&self, name: Name, mail_address: MailAddress) -> Result<()> {
        let user = User::new(name, mail_address);
        if self.user_service().exists(&user)? {
            bail!(MyError::internal_server_error("ユーザは既に存在しています"))
        }
        self.repo().save(user)
    }

    // 以下はドメインオブジェクトを直接返す場合の例
    // ドメインオブジェクトを公開する = アプリケーションサービスのクライアントが意図せぬ使い方をする危険性を持つ（ダメというわけではない）
    // pub fn get(&self, id: UserId) -> Result<User> {
    //     let user =self.repo().find_by_id(id)?.ok_or_else(|| MyError::internal_server_error("ユーザが見つかりませんでした"))?;
    //     Ok(user)
    // }

    // ドメインオブジェクトを直接返さず、DTOを介す例
    pub fn get(&self, id: UserId) -> Result<UserData> {
        let user = self
            .repo()
            .find_by_id(id)?
            .ok_or_else(|| MyError::internal_server_error("ユーザが見つかりませんでした"))?;
        Ok(user.into())
    }

    // 更新するパラメータが増えてもシグネチャを変えなくていいように、Command型を定義して引数とする
    pub fn update(&self, command: UserUpdateCommand) -> Result<()> {
        let mut user = self
            .repo()
            .find_by_id(command.id().clone())?
            .ok_or_else(|| MyError::internal_server_error("ユーザが見つかりませんでした"))?;

        if let Some(name) = command.name() {
            user.change_name(name.clone());
            if self.user_service().exists(&user)? {
                bail!(MyError::internal_server_error("ユーザは既に存在しています"))
            }
        }

        if let Some(mail_address) = command.mail_address() {
            user.change_mail_address(mail_address.clone());
        }

        self.repo().save(user)?;
        Ok(())
    }

    pub fn delete(&self, command: UserDeleteCommand) -> Result<()> {
        let target_id = command.id().clone();
        let user = self
            .repo()
            .find_by_id(target_id)?
            .ok_or_else(|| MyError::internal_server_error("ユーザが見つかりませんでした"))?;
        self.repo.delete(user)?;
        Ok(())
    }
}

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

// Updateメソッドのパラメータ群。Optionで定義したフィールドは任意の更新項目とする。
#[derive(Clone, Debug, Getters, new)]
pub struct UserUpdateCommand {
    id: UserId,
    name: Option<Name>,
    mail_address: Option<MailAddress>,
}

#[derive(Clone, Debug, Getters, new)]
pub struct UserDeleteCommand {
    id: UserId,
}
