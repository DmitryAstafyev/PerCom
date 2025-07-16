use crate::scheme::{provider::Provider, users::model::*};

/// Trait for managing user-related resources and basic authentication logic.
///
/// This trait extends the base [`Provider`] trait and defines core operations for handling user entities,
/// as well as a simple method for token-based validation (e.g., for request authorization).
///
/// It serves as the backend abstraction behind the `/users` API endpoints and supports future extensibility
/// for authentication and user management features.
///
/// # Methods
///
/// - [`get_all`] — Returns all users.
/// - [`get`] — Retrieves a user by ID.
/// - [`create`] — Creates a new user from input data.
/// - [`is_token_valid`] — Verifies the validity of an authorization token.
///
/// # Notes
/// - This trait is intentionally minimal and can be expanded to support password auth, roles, profiles, etc.
/// - The `is_token_valid` method can be used by request extractors like [`AuthToken`] to perform authentication checks.
pub trait UsersProvider: Provider {
    /// Returns a list of all users.
    fn get_all(&self) -> Vec<User>;

    /// Returns a user by ID, or `None` if not found.
    fn get(&self, id: &str) -> Option<User>;

    /// Creates a new user and returns the resulting object.
    fn create(&self, input: UserInput) -> User;

    /// Validates the given token.
    ///
    /// Returns `true` if the token is considered valid; otherwise, `false`.
    fn is_token_valid(&self, _token: &str) -> bool;
}
