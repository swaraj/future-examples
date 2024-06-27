use std::{fmt, thread};

use chrono::Utc;

/// Poor man's logging library

/// This func will do actual logging
pub fn log<S: AsRef<str> + fmt::Display>(s: S) {
    println!(
        "{} {:?}||{:?} {}",
        Utc::now().to_rfc3339(),
        thread::current().id(),
        thread::current().name(),
        s
    );
}

/// Logging with LEVEL as param
pub fn log_at_level<S: AsRef<str> + fmt::Display>(level: &str, s: S) {
    log(format!("{} {}", level, s));
}

/// Logging at ERROR Level
pub fn error<S: AsRef<str> + fmt::Display>(s: S) {
    log_at_level(format!("\x1B[1;31m{}\x1B[0m", "ERROR").as_str(), s);
}

/// Logging at DEBUG Level
pub fn debug<S: AsRef<str> + fmt::Display>(s: S) {
    log_at_level(format!("\x1B[1;32m{}\x1B[0m", "DEBUG").as_str(), s);
}

/// Logging at INFO Level
pub fn info<S: AsRef<str> + fmt::Display>(s: S) {
    log_at_level(format!("\x1B[1;34m{}\x1B[0m", "INFO").as_str(), s);
}
