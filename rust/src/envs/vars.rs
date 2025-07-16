use std::{env, net::SocketAddr};

/// Name of the environment variable used to configure the server's bind address.
const RUST_SERVER_ADDR_ENVVAR: &str = "RUST_SERVER_ADDR";

/// Default address and port used by the server if the environment variable is not set.
const RUST_SERVER_DEFAULT_ADDR: &str = "0.0.0.0:8080";

/// Retrieves the socket address the server should bind to.
///
/// This function checks the `RUST_SERVER_ADDR` environment variable to determine the address
/// and port the server should listen on. If the variable is not set, the default address
/// `0.0.0.0:8080` is used.
///
/// # Returns
/// A `SocketAddr` indicating where the server should bind.
///
/// # Errors
/// Returns an `io::Error` if the provided address cannot be parsed as a valid `SocketAddr`.
pub fn get_server_addr() -> std::io::Result<SocketAddr> {
    env::var(RUST_SERVER_ADDR_ENVVAR)
        .unwrap_or(RUST_SERVER_DEFAULT_ADDR.to_owned())
        .parse::<SocketAddr>()
        .map_err(|err| std::io::Error::other(err.to_string()))
}

#[cfg(test)]
/// Name of the environment variable used during testing to configure the target server address.
const RUST_CLIENT_ADDR_ENVVAR: &str = "RUST_CLIENT_ADDR";

#[cfg(test)]
/// Default server address used during testing if the `RUST_CLIENT_ADDR` environment variable is not set.
const RUST_CLIENT_DEFAULT_ADDR: &str = "127.0.0.1:8080";

#[cfg(test)]
/// Returns the URL of the target server used during tests.
///
/// This function reads the `RUST_CLIENT_ADDR` environment variable to determine where the client
/// should send requests during integration or property-based tests. If the variable is not set,
/// it defaults to `127.0.0.1:8080`.
///
/// # Returns
/// A `String` representing the base address of the server used during testing (e.g., `127.0.0.1:8080`).
pub fn get_client_url() -> String {
    env::var(RUST_CLIENT_ADDR_ENVVAR).unwrap_or(RUST_CLIENT_DEFAULT_ADDR.to_owned())
}

#[cfg(test)]
/// If set in "1", will write test data into file in $TEMP folder
const WRITE_TEST_RESULT_TO_FILE: &str = "WRITE_TEST_RESULT_TO_FILE";

#[cfg(test)]
pub fn write_test_data() -> bool {
    env::var(WRITE_TEST_RESULT_TO_FILE)
        .map(|v| v == "1")
        .unwrap_or(false)
}
