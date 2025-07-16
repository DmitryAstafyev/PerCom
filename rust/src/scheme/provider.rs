/// Base trait for all provider implementations, regardless of the specific API resource they handle.
///
/// This trait serves as a common abstraction layer for components that supply or manage data used
/// in request handling logic (e.g., posts, users, etc.).
///
/// All implementors must be both `Send` and `Sync`, ensuring they can be safely shared across threads.
pub trait Provider: Send + Sync {}
