//! A module for handling command line exits with custom messages and exit
//! codes.
//!
//! This module provides a convenient way to handle command line exits with
//! custom messages and exit codes.
use std::process::exit;

/// Represents a command exit object with a custom message and exit code.
#[derive(Debug)]
pub struct CmdResult {
    /// The exit code associated with the exit.
    pub code: i32,
    /// The optional message associated with the exit.
    pub message: Option<String>,
}

impl CmdResult {
    /// Creates a new [`CmdExit`] instance with an error message and exit code 1.
    #[must_use]
    pub fn error_with_message(message: &str) -> Self {
        Self {
            code: 1,
            message: Some(format!("❗ {message}")),
        }
    }
    /// Creates a new [`CmdExit`] instance exit code 1.
    #[must_use]
    pub const fn error() -> Self {
        Self {
            code: 1,
            message: None,
        }
    }

    /// Creates a new [`CmdExit`] instance with a success message and exit code 0.
    #[must_use]
    pub fn ok_with_message(message: &str) -> Self {
        Self {
            code: 0,
            message: Some(message.to_string()),
        }
    }

    /// Creates a new [`CmdExit`] instance with a success message and exit code 0 without any message.
    #[must_use]
    pub const fn ok() -> Self {
        Self {
            code: 0,
            message: None,
        }
    }

    /// Exits the command line with the configured message and exit code.
    pub fn exit(&self) {
        if let Some(message) = &self.message {
            eprintln!("{message}");
        };

        exit(self.code);
    }
}
