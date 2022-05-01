//! A module designed for error handling while printing to the standard output.

use std::io;

/// Contains the kind of operation that was being performed when the error occurred.
#[derive(PartialEq, Debug, Clone)]
pub enum PrintErrorKind {
    /// Could not write to the standard output.
    Write,

    /// Could not flush the standard output buffer.
    Flush,
}

/// An error that occurs when printing to the standard output.
pub struct PrintError {
    error: io::Error,
    kind: PrintErrorKind,
}

impl PrintError {
    /// Get a reference to the [io::Error].
    pub fn error(&self) -> &io::Error {
        &self.error
    }

    /// Get the kind of error that occurred.
    pub fn kind(&self) -> &PrintErrorKind {
        &self.kind
    }

    /// Consume `self` and return the [io::Error].
    pub fn take_error(self) -> io::Error {
        self.error
    }

    /// Create a new [PrintError].
    pub fn new(error: io::Error, kind: PrintErrorKind) -> Self {
        Self { error, kind }
    }
}