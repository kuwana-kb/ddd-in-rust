use anyhow::Result;

use crate::{
    c8_user_interface::{
        domain::{exists, HaveUserRepository, MailAddress, Name, User, UserId, UserRepository},
        usecase::{UserDeleteCommand, UserUpdateCommand, UserData},
    },
    MyError,
};

pub trait UserApplicationService: HaveUserRepository + std::marker::Sized {
    fn register(&self, name: Name, mail_address: MailAddress) -> Result<()> {
        let user = User::new(name, mail_address);
        if exists(self, &user)? {
            bail!(MyError::internal_server_error("ユーザは既に存在しています"))
        }
        self.provide_user_repository().save(user)
    }

    fn get(&self, id: UserId) -> Result<UserData> {
        let user = self
            .provide_user_repository()
            .find_by_id(id)?
            .ok_or_else(|| MyError::internal_server_error("ユーザが見つかりませんでした"))?;
        Ok(user.into())
    }

    fn update(&self, command: UserUpdateCommand) -> Result<()> {
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

    fn delete(&self, command: UserDeleteCommand) -> Result<()> {
        let target_id = command.id().clone();
        let user = self
            .provide_user_repository()
            .find_by_id(target_id)?
            .ok_or_else(|| MyError::internal_server_error("ユーザが見つかりませんでした"))?;
        self.provide_user_repository().delete(user)?;
        Ok(())
    }
}

impl<T: HaveUserRepository> UserApplicationService for T {}

pub trait HaveUserApplicationService {
    type UserApplicationService: UserApplicationService;

    fn provide_user_service(&self) -> &Self::UserApplicationService;
}
