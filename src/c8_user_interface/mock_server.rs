use super::{HaveUserApplicationService, HaveUserRepository, MockContext};

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
