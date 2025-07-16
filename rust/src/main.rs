#[cfg(test)]
mod tests;

pub(crate) mod envs;
pub(crate) mod scheme;
mod state;

use actix_web::{App, HttpServer, web};

use crate::envs::vars::get_server_addr;

/// Launches the HTTP server and binds the route handlers for two resource families: `/posts` and `/users`.
///
/// The `/posts` endpoints implement the required functionality as defined in the original OpenAPI specification,
/// and are fully covered by the automated test suite using property-based testing (`proptest`).
///
/// The `/users` endpoints are included as an example to demonstrate how the project can be extended with additional
/// resource groups. These endpoints are not covered by tests and are meant for illustrative purposes only.
///
/// This is the main entry point of the application, executed using the Actix-Web asynchronous runtime.
///
/// # Returns
/// Returns an `std::io::Result<()>` indicating whether the server launched successfully or encountered an I/O error.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init logs
    let guard = envs::logs::init()?;
    // Create providers
    let users_provider = scheme::users::DummyProvider::wrapped();
    let posts_provider = scheme::posts::DummyProvider::wrapped();
    // Create global states
    let global_state = web::Data::new(state::GlobalServerState::new(users_provider.clone()));
    // Create local/context states
    let posts_state = web::Data::new(scheme::posts::routes::PostsState::new(posts_provider));
    let users_state = web::Data::new(scheme::users::routes::UsersState::new(users_provider));
    HttpServer::new(move || {
        App::new()
            // Create global state
            .app_data(global_state.clone())
            .service(
                web::scope("/posts")
                    // Create local state
                    .app_data(posts_state.clone())
                    .configure(scheme::posts::routes::configure),
            )
            .service(
                web::scope("/users")
                    // Create local state
                    .app_data(users_state.clone())
                    .configure(scheme::users::routes::configure),
            )
    })
    .bind(get_server_addr()?)?
    .run()
    .await?;

    // Technically it's useless, but it helps to remember `guard` should live until end of application
    drop(guard);

    Ok(())
}
