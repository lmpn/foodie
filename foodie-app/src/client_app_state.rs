use crate::api::authorization_api::AuthenticatedUser;

#[derive(Default)]
pub struct ClientAppState {
    pub user: Option<AuthenticatedUser>,
}

impl ClientAppState {
    pub fn new(user: Option<AuthenticatedUser>) -> Self {
        Self { user }
    }
}
