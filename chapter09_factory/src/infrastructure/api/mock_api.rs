use derive_new::new;

use crate::{
    domain::{HaveUserFactory, HaveUserRepository},
    infrastructure::MockContext,
    usecase::HaveUserApplicationService,
};

#[derive(Clone, Debug, new)]
pub struct MockApi {
    context: MockContext,
}

impl HaveUserFactory for MockApi {
    type UserFactory = MockContext;

    fn provide_user_factory(&self) -> &Self::UserFactory {
        &self.context
    }
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
