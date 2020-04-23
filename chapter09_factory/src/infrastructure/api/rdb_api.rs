use derive_new::new;

use crate::{
    domain::{HaveUserFactory, HaveUserRepository},
    infrastructure::RDBContext,
    usecase::HaveUserApplicationService,
};

#[derive(Clone, Debug, new)]
pub struct RDBApi {
    context: RDBContext,
}

impl HaveUserFactory for RDBApi {
    type UserFactory = RDBContext;

    fn provide_user_factory(&self) -> &Self::UserFactory {
        &self.context
    }
}

impl HaveUserRepository for RDBApi {
    type UserRepository = RDBContext;

    fn provide_user_repository(&self) -> &Self::UserRepository {
        &self.context
    }
}

impl HaveUserApplicationService for RDBApi {
    type UserApplicationService = Self;

    fn provide_user_service(&self) -> &Self::UserApplicationService {
        self
    }
}
