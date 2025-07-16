use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use std::sync::Arc;
use tracing::debug;

use crate::scheme::{auth::AuthToken, posts::*};

/// Shared application state for the `/posts` route group.
///
/// This wrapper holds a thread-safe, reference-counted instance of a type implementing the [`PostsProvider`] trait.
/// It is injected into Actix-Web handlers using `web::Data` for shared access across requests.
#[derive(Clone)]
pub struct PostsState {
    /// The backend provider that implements all operations for managing blog posts.
    pub provider: Arc<dyn PostsProvider>,
}

impl PostsState {
    /// Constructs a new [`PostsState`] with the given provider.
    ///
    /// # Parameters
    /// - `provider`: An `Arc`-wrapped implementation of [`PostsProvider`]
    ///
    /// # Returns
    /// A new [`PostsState`] instance.
    pub fn new(provider: Arc<dyn PostsProvider>) -> Self {
        Self { provider }
    }
}

/// Handles `GET /posts`
///
/// Returns a JSON array containing all available posts.
///
/// # Response
/// - `200 OK` with JSON array of [`Post`] objects
#[get("")]
async fn list_posts(state: web::Data<PostsState>) -> impl Responder {
    let posts = state.provider.get_all();
    HttpResponse::Ok().json(posts)
}

/// Handles `POST /posts`
///
/// Creates a new blog post from the request body.
/// Requires a valid [`AuthToken`] (simulated in this implementation).
///
/// # Request Body
/// Expects a JSON payload conforming to [`PostInput`].
///
/// # Response
/// - `201 Created` with the created [`Post`] as JSON
/// - `Location` header pointing to the newly created resource
#[post("")]
async fn create_post(
    _auth: AuthToken,
    state: web::Data<PostsState>,
    body: web::Json<PostInput>,
) -> impl Responder {
    debug!("Request: create post");
    let post = state.provider.create(body.into_inner());
    HttpResponse::Created()
        .append_header(("Location", format!("/posts/{}", post.id)))
        .json(post)
}

/// Handles `GET /posts/{id}`
///
/// Retrieves a blog post by its ID.
///
/// # Path Parameters
/// - `id`: The unique identifier of the post
///
/// # Response
/// - `200 OK` with the post as JSON
/// - `404 Not Found` if the post does not exist
#[get("/{id}")]
async fn get_post(state: web::Data<PostsState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    debug!("Request: get post {}", id);
    match state.provider.get(&id) {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().finish(),
    }
}

/// Handles `PUT /posts/{id}`
///
/// Updates an existing blog post with new data.
/// Requires a valid [`AuthToken`] (simulated).
///
/// # Path Parameters
/// - `id`: The ID of the post to update
///
/// # Request Body
/// JSON payload matching [`PostInput`]
///
/// # Response
/// - `200 OK` with updated post
/// - `404 Not Found` if the post does not exist
#[put("/{id}")]
async fn update_post(
    _auth: AuthToken,
    state: web::Data<PostsState>,
    path: web::Path<String>,
    body: web::Json<PostInput>,
) -> impl Responder {
    let id = path.into_inner();
    debug!("Request: update post {}", id);
    match state.provider.update(&id, body.into_inner()) {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().finish(),
    }
}

/// Handles `DELETE /posts/{id}`
///
/// Deletes a blog post by ID.
/// Requires a valid [`AuthToken`] (simulated).
///
/// # Path Parameters
/// - `id`: The ID of the post to delete
///
/// # Response
/// - `204 No Content` if deletion was successful
/// - `404 Not Found` if the post does not exist
#[delete("/{id}")]
async fn delete_post(
    _auth: AuthToken,
    state: web::Data<PostsState>,
    path: web::Path<String>,
) -> impl Responder {
    if state.provider.delete(&path.into_inner()) {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

/// Registers all `/posts` route handlers into the Actix-Web service configuration.
///
/// This function should be called from the main application setup to bind
/// the `/posts` resource group to its corresponding handlers.
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(list_posts);
    cfg.service(create_post);
    cfg.service(get_post);
    cfg.service(update_post);
    cfg.service(delete_post);
}
