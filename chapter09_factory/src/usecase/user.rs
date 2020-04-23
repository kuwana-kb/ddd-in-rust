use anyhow::Result;

use crate::{
    domain::{HaveUserFactory, HaveUserRepository, UserFactory, UserRepository},
    usecase::CreateUserCommand,
};

pub trait UserApplicationService:
    HaveUserRepository + HaveUserFactory + std::marker::Sized
{
    fn register(&self, cmd: CreateUserCommand) -> Result<()> {
        let name = cmd.name().clone();
        let user = self.provide_user_factory().create(name)?;
        self.provide_user_repository().save(user)
    }
}

// Repositoryを持つものに対して自動で実装を与えられる
impl<T: HaveUserRepository + HaveUserFactory> UserApplicationService for T {}

pub trait HaveUserApplicationService {
    type UserApplicationService: UserApplicationService;

    fn provide_user_service(&self) -> &Self::UserApplicationService;
}
