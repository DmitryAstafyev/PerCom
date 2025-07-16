use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

/// Name of the application directory, relative to the user's home directory or system temp directory.
const APP_DIR: &str = ".ex_server";

/// Name of the subdirectory where log files are stored.
const LOG_DIR: &str = "logs";

/// Returns the base application directory path, creating it if necessary.
///
/// By default, this function constructs the path `$HOME/.ex_server`. If the `$HOME` directory is unavailable
/// (e.g., in a restricted or containerized environment), the system's temporary directory is used instead,
/// resulting in `$TEMP/.ex_server`.
///
/// If the target directory does not exist, it is created recursively using `fs::create_dir_all`.
///
/// # Returns
/// A [`PathBuf`] representing the base directory used by the application.
///
/// # Errors
/// Returns an `io::Error` if the directory cannot be created.
pub fn get_home() -> io::Result<PathBuf> {
    // By default we are trying to put logs into `$HOME/.ex_server/logs`
    // If by some reasons $HOME isn't available - we are using `$TEMP` folder
    let path = env::home_dir().unwrap_or(env::temp_dir()).join(APP_DIR);
    if !Path::new(&path).exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}

/// Returns the log directory path, creating it if necessary.
///
/// The log directory is expected to be a subdirectory named `logs` inside the application base directory,
/// e.g., `$HOME/.ex_server/logs` or `$TEMP/.ex_server/logs` depending on availability of `$HOME`.
///
/// If the log directory does not exist, it is created recursively using `fs::create_dir_all`.
///
/// # Returns
/// A [`PathBuf`] pointing to the directory where log files should be written.
///
/// # Errors
/// Returns an `io::Error` if the base directory or the log directory cannot be created.
pub fn get_logs() -> io::Result<PathBuf> {
    let path = get_home()?.join(LOG_DIR);
    if !Path::new(&path).exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}
