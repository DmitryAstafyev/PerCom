use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

use crate::scheme::{posts::*, provider::Provider};

/// In-memory implementation of the [`PostsProvider`] trait for testing and demonstration purposes.
///
/// This provider stores posts in a thread-safe in-memory `HashMap`, protected by an `RwLock`.
/// It does not persist data and is intended for lightweight usage such as unit testing,
/// property-based testing, or examples.
///
/// Each method acquires a read or write lock on the underlying store to ensure safe access
/// across multiple threads.
///
/// # Concurrency
/// Internally uses `Arc<RwLock<HashMap<String, Post>>>`, which allows shared access from multiple threads
/// with consistent data visibility.
///
/// # Limitations
/// - Data is not persisted between runs.
/// - Not optimized for large-scale production use.
pub struct DummyProvider {
    store: RwLock<HashMap<String, Post>>,
}

impl DummyProvider {
    /// Constructs a new `DummyProvider` instance without wrapping it in an `Arc`.
    ///
    /// This constructor is mainly useful for local or internal usage when shared ownership is not required.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
        }
    }

    /// Constructs a new `DummyProvider` instance and wraps it in an `Arc`.
    ///
    /// This is the recommended way to instantiate the provider in contexts where shared ownership is needed,
    /// such as within Actix-Web app data or multithreaded test runners.
    pub fn wrapped() -> Arc<Self> {
        Arc::new(Self {
            store: RwLock::new(HashMap::new()),
        })
    }
}

impl Provider for DummyProvider {}

impl PostsProvider for DummyProvider {
    /// Returns all stored posts as a `Vec<Post>`, cloned from the internal map.
    fn get_all(&self) -> Vec<Post> {
        self.store.read().unwrap().values().cloned().collect()
    }

    /// Returns the post with the specified ID, if it exists.
    fn get(&self, id: &str) -> Option<Post> {
        self.store.read().unwrap().get(id).cloned()
    }

    /// Creates a new post from the given input and stores it under a generated UUID.
    ///
    /// The generated post is returned.
    fn create(&self, input: PostInput) -> Post {
        let id = Uuid::new_v4().to_string();
        let post = Post {
            id: id.clone(),
            author: input.author,
            date: input.date,
            content: input.content,
        };
        self.store.write().unwrap().insert(id.clone(), post.clone());
        post
    }

    /// Updates an existing post with the specified ID, replacing it with the provided input.
    ///
    /// Returns the updated post if the ID exists, or `None` otherwise.
    fn update(&self, id: &str, input: PostInput) -> Option<Post> {
        let mut store = self.store.write().unwrap();
        if store.contains_key(id) {
            let post = Post {
                id: id.to_string(),
                author: input.author,
                date: input.date,
                content: input.content,
            };
            store.insert(id.to_string(), post.clone());
            Some(post)
        } else {
            None
        }
    }

    /// Deletes the post with the given ID.
    ///
    /// Returns `true` if the post existed and was removed, or `false` if the ID was not found.
    fn delete(&self, id: &str) -> bool {
        self.store.write().unwrap().remove(id).is_some()
    }
}
