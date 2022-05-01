//! A module for writing to the standard output.

use std::io;
use std::io::{Stdout, Write};

use crate::err::{PrintError, PrintErrorKind};

/// A struct that provides methods for interacting with the standard output.
pub struct StdoutWriter {
    stdout: Stdout,
}

impl StdoutWriter {
    /// Write the given input to the standard output.
    /// If an error occurs, return a new [PrintError] with the error kind [PrintErrorKind::Write].
    pub fn write_all<B: AsRef<[u8]>>(&mut self, bytes: B) -> Result<(), PrintError> {
        self.stdout
            .write_all(bytes.as_ref())
            .map_err(|err| PrintError::new(err, PrintErrorKind::Write))
    }

    /// Flush the buffer of the inner [Stdout].
    /// If an error occurs, return a new [PrintError] with the error kind [PrintErrorKind::Flush].
    pub fn flush(&mut self) -> Result<(), PrintError> {
        self.stdout
            .flush()
            .map_err(|err| PrintError::new(err, PrintErrorKind::Flush))
    }

    /// Write and flush the given input to the inner [Stdout].
    /// Returns a [PrintError] on error.
    pub fn print<B: AsRef<[u8]>>(&mut self, bytes: B) -> Result<(), PrintError> {
        self.write_all(bytes.as_ref())?;
        self.flush()
    }

    /// Print the given input to the standard output, followed by the newline character.
    /// Returns a [PrintError] on error.
    pub fn println<B: AsRef<[u8]>>(&mut self, bytes: B) -> Result<(), PrintError> {
        self.write_all(bytes.as_ref())?;
        self.print(b"\n")
    }

    /// Create a new [StdoutWriter].
    pub fn new(stdout: Stdout) -> Self {
        Self { stdout }
    }
}

impl Default for StdoutWriter {
    /// Create a new [StdoutWriter].
    fn default() -> Self {
        Self::new(io::stdout())
    }
}

/// A utility method for printing the given input to the standard output.
/// It is not advised to call this method more than once at a time.
/// Instead, create a new [StdoutWriter] and then call [StdoutWriter::print] without concern.
pub fn print<B: AsRef<[u8]>>(bytes: B) -> Result<(), PrintError> {
    let mut writer = StdoutWriter::default();
    writer.print(bytes)
}

/// A utility method for printing the given input, followed by
/// the newline character to the standard output.
/// It is not advised to call this method more than once at a time.
/// Instead, create a new [StdoutWriter] and then call [StdoutWriter::println] without concern.
pub fn println<B: AsRef<[u8]>>(bytes: B) -> Result<(), PrintError> {
    let mut writer = StdoutWriter::default();
    writer.println(bytes)
}