use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

use crate::scheme::{provider::Provider, users::*};

/// In-memory implementation of the [`UsersProvider`] trait for testing and demonstration.
///
/// This provider uses a thread-safe `HashMap` to store user records in memory.  
/// It does not perform any persistent storage and is not intended for production use.
///
/// Token validation is stubbed to always return `true`, simulating an "authenticated" request.
///
/// # Purpose
/// - To demonstrate how the `/users` endpoint group could be implemented.
/// - To support structural completeness and showcase extensibility.
///
/// # Concurrency
/// Internally guarded by `RwLock` to allow safe concurrent read/write access from multiple threads.
pub struct DummyProvider {
    store: RwLock<HashMap<String, User>>,
}

impl DummyProvider {
    /// Creates a new instance of `DummyProvider` (unwrapped).
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
        }
    }
    /// Creates a new `DummyProvider` wrapped in an `Arc`.
    ///
    /// Useful for sharing across threads or injecting into Actix-Web app state.
    pub fn wrapped() -> Arc<Self> {
        Arc::new(Self {
            store: RwLock::new(HashMap::new()),
        })
    }
}

impl Provider for DummyProvider {}

impl UsersProvider for DummyProvider {
    /// Returns all stored users.
    fn get_all(&self) -> Vec<User> {
        self.store.read().unwrap().values().cloned().collect()
    }

    /// Returns a user by ID, if present.
    fn get(&self, id: &str) -> Option<User> {
        self.store.read().unwrap().get(id).cloned()
    }

    /// Creates a new user with a generated UUID and stores it.
    ///
    /// The resulting `User` is returned.
    fn create(&self, input: UserInput) -> User {
        let id = Uuid::new_v4().to_string();
        let post = User {
            id: id.clone(),
            nickname: input.nickname,
            email: input.email,
        };
        self.store.write().unwrap().insert(id.clone(), post.clone());
        post
    }

    /// Always returns `true` as a placeholder implementation.
    ///
    /// This method simulates successful token validation for all inputs.
    fn is_token_valid(&self, _token: &str) -> bool {
        true
    }
}
