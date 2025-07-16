use std::sync::Arc;

use crate::scheme::users::UsersProvider;

#[derive(Clone)]
pub struct GlobalServerState {
    pub provider: Arc<dyn UsersProvider>,
}

impl GlobalServerState {
    pub fn new(provider: Arc<dyn UsersProvider>) -> GlobalServerState {
        Self { provider }
    }
    pub fn is_token_valid<S: AsRef<str>>(&self, token: S) -> bool {
        self.provider.is_token_valid(token.as_ref())
    }
}
