use crate::scheme::{posts::model::*, provider::Provider};

/// Trait for managing blog post resources, providing basic CRUD operations.
///
/// This trait extends the [`Provider`] base trait and defines the full set of operations
/// needed to manage `Post` entities, including listing, retrieval, creation, updating,
/// and deletion. It serves as the backend abstraction behind the `/posts` API endpoints.
///
/// Implementors can define how data is stored or retrieved (e.g., in-memory, database, etc.).
///
/// All methods are synchronous and expected to be cheap and fast for in-memory use cases.
/// For I/O-bound implementations (e.g., database-backed), async variants might be preferable.
///
/// # Methods
///
/// - [`get_all`] – Returns all available posts.
/// - [`get`] – Retrieves a specific post by ID.
/// - [`create`] – Creates a new post from the given input.
/// - [`update`] – Updates an existing post, if found.
/// - [`delete`] – Removes a post by ID, returning success status.
pub trait PostsProvider: Provider {
    /// Returns a list of all posts.
    fn get_all(&self) -> Vec<Post>;

    /// Returns a post by ID, or `None` if not found.
    fn get(&self, id: &str) -> Option<Post>;

    /// Creates a new post and returns it, including the generated ID.
    fn create(&self, input: PostInput) -> Post;

    /// Updates an existing post by ID, returning the updated post if successful.
    fn update(&self, id: &str, input: PostInput) -> Option<Post>;

    /// Deletes a post by ID. Returns `true` if a post was deleted.
    fn delete(&self, id: &str) -> bool;
}
