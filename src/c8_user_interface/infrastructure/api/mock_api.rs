use derive_new::new;

use crate::c8_user_interface::{
    domain::HaveUserRepository, infrastructure::MockContext, usecase::HaveUserApplicationService,
};

#[derive(Clone, Debug, new)]
pub struct MockApi {
    context: MockContext,
}

impl HaveUserRepository for MockApi {
    type UserRepository = MockContext;

    fn provide_user_repository(&self) -> &Self::UserRepository {
        &self.context
    }
}

impl HaveUserApplicationService for MockApi {
    type UserApplicationService = Self;

    fn provide_user_service(&self) -> &Self::UserApplicationService {
        self
    }
}
