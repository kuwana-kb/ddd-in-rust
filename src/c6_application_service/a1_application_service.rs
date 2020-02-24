use std::str::FromStr;

use anyhow::Result;
use derive_getters::Getters;
use derive_more::Display;
use derive_new::new;
use uuid::Uuid;

use crate::MyError;

// -------------------------
// Entity & Value Object
// -------------------------

#[derive(Clone, Debug, Getters)]
pub struct User {
    id: UserId,
    name: Name,
    mail_address: MailAddress,
}

impl User {
    // はじめてインスタンスを生成する際に利用する
    pub fn new(name: Name, mail_address: MailAddress) -> Self {
        Self {
            id: UserId::default(),
            name,
            mail_address,
        }
    }

    // インスタンスを再構成する際に利用する
    pub fn rebuild(id: UserId, name: Name, mail_address: MailAddress) -> Self {
        Self {
            id,
            name,
            mail_address,
        }
    }

    pub fn change_name(&mut self, name: Name) {
        self.name = name;
    }

    pub fn change_mail_address(&mut self, mail_address: MailAddress) {
        self.mail_address = mail_address;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub struct UserId(String);

impl Default for UserId {
    fn default() -> Self {
        UserId(Uuid::new_v4().to_string())
    }
}

impl FromStr for UserId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // ここでは省いているが、idの制約を満たすかチェックすること
        Ok(UserId(s.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub struct Name(String);

impl FromStr for Name {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.chars().count() < 3 || s.chars().count() > 20 {
            bail!(MyError::type_error("ユーザ名は3文字以上、20文字以下です"))
        }
        Ok(Name(s.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub struct MailAddress(String);

// -------------------------
// Domain Service
// -------------------------

#[derive(Clone, Debug, Getters)]
pub struct UserService<Repo: IUserRepository> {
    repo: Repo,
}

impl<Repo> UserService<Repo>
where
    Repo: IUserRepository,
{
    pub fn exists(&self, user: &User) -> Result<bool> {
        let duplicated_user = self.repo().find_by_name(user.clone().name)?;
        match duplicated_user {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}

// -------------------------
// Repository
// -------------------------

pub trait IUserRepository {
    fn find_by_id(&self, id: UserId) -> Result<Option<User>>;

    fn find_by_name(&self, name: Name) -> Result<Option<User>>;

    fn save(&self, user: User) -> Result<()>;

    fn delete(&self, user: User) -> Result<()>;
}

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
        let target_id = command.id();
        let user = self.repo().find_by_id(target_id)?.ok_or_else(|| MyError::internal_server_error("ユーザが見つかりませんでした"))?;
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
pub sturct UserDeleteCommand {
    id: UserId,
}
