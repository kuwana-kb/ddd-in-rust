use anyhow::Result;
use common::MyError;

use crate::{
    domain::{exists, HaveUserRepository, Name, User, UserRepository},
    usecase::{CreateUserCommand, DeleteUserCommand, UpdateUserCommand, UserData},
};

// Cake Patternによる実装
// c6ではApplicationServiceをstructで表現したが、今回のパターンではtraitで表現している
// このパターンだとtrait上のデフォルト実装に加えて、独自の実装に変更することもできる
//
// パターンは以下のブログを参考にした
// https://keens.github.io/blog/2017/12/01/rustnodi/
pub trait UserApplicationService: HaveUserRepository + std::marker::Sized {
    fn register(&self, cmd: CreateUserCommand) -> Result<()> {
        let user = User::new(cmd.name().clone(), cmd.mail_address().clone());
        if exists(self, &user)? {
            bail!(MyError::internal_server_error("ユーザは既に存在しています"))
        }
        self.provide_user_repository().save(user)
    }

    fn get_by_name(&self, name: Name) -> Result<UserData> {
        let user = self
            .provide_user_repository()
            .find_by_name(name)?
            .ok_or_else(|| MyError::internal_server_error("ユーザが見つかりませんでした"))?;
        Ok(user.into())
    }

    fn update(&self, command: UpdateUserCommand) -> Result<()> {
        let mut user = self
            .provide_user_repository()
            .find_by_id(command.id().clone())?
            .ok_or_else(|| MyError::internal_server_error("ユーザが見つかりませんでした"))?;

        if let Some(name) = command.name() {
            user.change_name(name.clone());
            if exists(self, &user)? {
                bail!(MyError::internal_server_error("ユーザは既に存在しています"))
            }
        }

        if let Some(mail_address) = command.mail_address() {
            user.change_mail_address(mail_address.clone());
        }

        self.provide_user_repository().save(user)?;
        Ok(())
    }

    fn delete(&self, command: DeleteUserCommand) -> Result<()> {
        let target_id = *command.id();
        let user = self
            .provide_user_repository()
            .find_by_id(target_id)?
            .ok_or_else(|| MyError::internal_server_error("ユーザが見つかりませんでした"))?;
        self.provide_user_repository().delete(user)?;
        Ok(())
    }
}

// Repositoryを持つものに対して自動で実装を与えられる
impl<T: HaveUserRepository> UserApplicationService for T {}

pub trait HaveUserApplicationService {
    type UserApplicationService: UserApplicationService;

    fn provide_user_service(&self) -> &Self::UserApplicationService;
}
