//! The main file for the hello-world project.

use std::{io, process};
use std::env;
use std::io::{Stdout, Write};

use crate::err::{PrintError, PrintErrorKind};

pub mod err;
pub mod exit_code;

/// These bytes represent the UTF-8 string "Hello, world!\n"\
/// `\n` represents the console newline character.
pub const MESSAGE_BYTES: &[u8; 14] = b"Hello, world!\n";

/// A struct that provides methods for interacting with the standard output.
pub struct StdoutWriter {
    stdout: Stdout,
}

impl StdoutWriter {
    /// Write all the given bytes to the standard output.
    /// If an error occurs, return a new [PrintError] with the error kind [PrintErrorKind::Write].
    pub fn write_all(&mut self, message: &[u8]) -> Result<(), PrintError> {
        self.stdout
            .write_all(message)
            .map_err(|err| PrintError::new(err, PrintErrorKind::Write))
    }

    /// Flush the buffer of the inner [Stdout].
    /// If an error occurs, return a new [PrintError] with the error kind [PrintErrorKind::Flush].
    pub fn flush(&mut self) -> Result<(), PrintError> {
        self.stdout
            .flush()
            .map_err(|err| PrintError::new(err, PrintErrorKind::Flush))
    }

    /// Print the given message to the standard output.
    /// Calls [Self::write_all(message)] then [Self::flush].
    pub fn print(&mut self, message: &[u8]) -> Result<(), PrintError> {
        self.write_all(message)?;
        self.flush()
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

/// A utility method for printing the given bytes to the standard output.
/// It is not advised to call this method more than once at a time.
/// Instead, create a new [StdoutWriter] and then call [StdoutWriter::print] without concern.
pub fn print(bytes: &[u8]) -> Result<(), PrintError> {
    let mut writer = StdoutWriter::default();
    writer.print(bytes)
}

/// A utility method for printing the [MESSAGE_BYTES] array to the standard output.
/// It is not advised to call this method more than once at a time.
/// Instead, create a new [StdoutWriter] and then call [StdoutWriter::print] without concern.
pub fn print_hello_world() -> Result<(), PrintError> {
    crate::print(MESSAGE_BYTES)
}

/// If there are no arguments, return normally, else exit the program with the exit code 1.
/// See [crate::exit_code::ARGUMENT_ERROR].
fn restrict_arguments() {
    // Get a copy of the program argument iterator.
    let arg_iter = env::args();

    // Since this program takes no arguments, it would be non-sensible to allow them to be supplied.
    // Therefore, if any arguments are supplied, the program will exit with a return code of 1.
    // (see [crate::exit_code::ARGUMENT_ERROR])
    if arg_iter.len() > 1 {
        // Skip the first argument, which is the program name, then
        // collect the rest of the arguments into a vector.
        let program_args = arg_iter.skip(1).collect::<Vec<String>>();

        eprintln!("unexpected arguments: {:?}", program_args);
        process::exit(exit_code::ARGUMENT_ERROR);
    }
}

/// The main function of the hello-world program.
/// See [crate::exit_code] for the possible exit codes of this program.
fn main() {
    // Make sure that the program is not being ran with any arguments.
    restrict_arguments();

    // Since we only print the message once, we can use the [crate::print_hello_world] method.
    process::exit(match crate::print_hello_world() {
        // The program completed without any errors.
        // Exit with a return code of 0. (see [crate::exit_code::OKAY])
        Ok(_) => exit_code::OKAY,

        // The program encountered an error while printing the message to the standard output.
        // Exit with a return code of -1. (see [crate::exit_code::OPERATION_ERROR])
        Err(err) => {
            let action = match err.kind() {
                PrintErrorKind::Write => "writing to the standard output",
                PrintErrorKind::Flush => "flushing the standard output buffer",
            };
            eprintln!("an error occurred while {}: {}", action, err.take_error());
            exit_code::OPERATION_ERROR
        }
    });
}