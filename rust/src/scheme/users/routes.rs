use actix_web::{HttpResponse, Responder, get, post, web};
use std::sync::Arc;

use crate::scheme::{auth::AuthToken, users::*};

/// Shared application state for the `/users` route group.
///
/// This wrapper holds a reference-counted implementation of the [`UsersProvider`] trait.
/// It is injected into Actix-Web handlers via `web::Data`, allowing concurrent access across requests.
#[derive(Clone)]
pub struct UsersState {
    /// Backend provider responsible for user-related operations.
    pub provider: Arc<dyn UsersProvider>,
}

impl UsersState {
    /// Constructs a new [`UsersState`] with the given provider.
    ///
    /// # Parameters
    /// - `provider`: An `Arc`-wrapped object implementing [`UsersProvider`].
    ///
    /// # Returns
    /// A new `UsersState` instance.
    pub fn new(provider: Arc<dyn UsersProvider>) -> Self {
        Self { provider }
    }
}

/// Handles `GET /users`
///
/// Requires a valid [`AuthToken`] to be present in the request.
///
/// Returns a list of all users stored in the system.
///
/// # Response
/// - `200 OK` with a JSON array of [`User`] objects
#[get("")]
async fn list_users(_auth: AuthToken, state: web::Data<UsersState>) -> impl Responder {
    let users = state.provider.get_all();
    HttpResponse::Ok().json(users)
}

/// Handles `POST /users`
///
/// Creates a new user from the submitted input.  
/// This endpoint does **not require authentication**.
///
/// # Request Body
/// Expects a JSON payload conforming to [`UserInput`].
///
/// # Response
/// - `201 Created` with the created [`User`] object
/// - Includes `Location` header with the URI of the created resource
#[post("")]
async fn create_user(state: web::Data<UsersState>, body: web::Json<UserInput>) -> impl Responder {
    let user = state.provider.create(body.into_inner());
    HttpResponse::Created()
        .append_header(("Location", format!("/users/{}", user.id)))
        .json(user)
}

/// Handles `GET /users/{id}`
///
/// Retrieves a specific user by ID. Requires a valid [`AuthToken`] to authorize the request.
///
/// # Path Parameters
/// - `id`: The identifier of the user to fetch
///
/// # Response
/// - `200 OK` with the corresponding [`User`] object
/// - `404 Not Found` if the user does not exist
#[get("/{id}")]
async fn get_user(
    _auth: AuthToken,
    state: web::Data<UsersState>,
    path: web::Path<String>,
) -> impl Responder {
    match state.provider.get(&path.into_inner()) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}

/// Registers the `/users` routes to the Actix-Web service configuration.
///
/// Should be called during application setup to attach all user-related handlers.
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users);
    cfg.service(create_user);
    cfg.service(get_user);
}
