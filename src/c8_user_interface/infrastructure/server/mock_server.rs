use crate::c8_user_interface::{
    domain::{HaveUserRepository},
    usecase::{HaveUserApplicationService},
    infrastructure::MockContext,
};

pub struct MockServer {
    context: MockContext,
}

impl HaveUserRepository for MockServer {
    type UserRepository = MockContext;

    fn provide_user_repository(&self) -> &Self::UserRepository {
        &self.context
    }
}

impl HaveUserApplicationService for MockServer {
    type UserApplicationService = Self;

    fn provide_user_service(&self) -> &Self::UserApplicationService {
        self
    }
}
