use chrono::prelude::*;
use std::io;
use tracing::debug;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt};

use crate::*;

/// Initializes the logging subsystem for the current server session.
///
/// A new log file is created for each run of the server. The log filename is based on the current UTC timestamp,
/// ensuring uniqueness and allowing for clear session separation. This approach is particularly suited for server
/// environments, where session-based logs help in debugging and post-mortem analysis.
///
/// Logging is configured using `tracing` and `tracing_appender`, with output directed to the new file in a
/// non-blocking fashion. The log level is determined via the `RUST_LOG` environment variable; if it is not set,
/// the default level is `debug`.
///
/// # Returns
/// Returns a `WorkerGuard` that must be held for the duration of the program to ensure proper flushing of log data.
///
/// # Errors
/// Returns an `io::Result::Err` if the log directory path cannot be determined or if any other I/O error occurs.
///
/// # Panics
/// Will panic if the `EnvFilter` cannot be created from the environment and the fallback filter creation fails.
pub fn init() -> io::Result<WorkerGuard> {
    let path = envs::paths::get_logs()?;
    let now = Utc::now();
    let filename = now.format("%Y%m%dT%H%M%S.logs").to_string();
    let file_appender = tracing_appender::rolling::never(&path, filename);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    fmt()
        .with_writer(non_blocking)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
        )
        .init();
    debug!("Log is inited at {}", now.to_rfc2822());
    Ok(guard)
}
