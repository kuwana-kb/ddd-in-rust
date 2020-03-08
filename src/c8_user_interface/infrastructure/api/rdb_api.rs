use derive_new::new;

use crate::c8_user_interface::{
    domain::HaveUserRepository, infrastructure::RDBContext, usecase::HaveUserApplicationService,
};

#[derive(Clone, Debug, new)]
pub struct RDBApi {
    context: RDBContext,
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
