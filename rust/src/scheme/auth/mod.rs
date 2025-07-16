use actix_web::{Error, FromRequest, HttpRequest, dev::Payload, web};
use futures_util::future::{Ready, ready};

use crate::state::GlobalServerState;

/// Represents an authorization token extracted from the `Authorization` header of an incoming HTTP request.
///
/// This is a minimal marker type used to gate access to protected endpoints via bearer token authentication.
/// If a request contains a valid token in the header, an instance of `AuthToken` is created and injected
/// into the handler. Otherwise, the request is rejected with a `401 Unauthorized` error.
///
/// This extractor is compatible with Actix-Web's request guards.
///
/// # Expected Header Format
/// ```text
/// Authorization: Bearer <token>
/// ```
///
/// # Failure Cases
/// - If the `Authorization` header is missing or malformed
/// - If the token is invalid or not recognized by the application state
#[derive(Debug, Default)]
pub struct AuthToken {}

impl FromRequest for AuthToken {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    /// Extracts the `AuthToken` from an HTTP request if the bearer token is present and valid.
    ///
    /// The token is retrieved from the `Authorization` header and validated against the global application state
    /// (`GlobalServerState`), which must be registered as application data.
    ///
    /// # Returns
    /// - `Ok(AuthToken)` if the header exists and the token is valid
    /// - `Err(ErrorUnauthorized)` if the token is missing or invalid
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .map(str::to_string);

        let auth_state = req.app_data::<web::Data<GlobalServerState>>().cloned();

        match (auth_header, auth_state) {
            (Some(token), Some(state)) => {
                if state.is_token_valid(token) {
                    ready(Ok(AuthToken::default()))
                } else {
                    ready(Err(actix_web::error::ErrorUnauthorized("Invalid token")))
                }
            }
            _ => ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized"))),
        }
    }
}
