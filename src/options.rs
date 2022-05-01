//! A module for parsing the command line arguments.

use std::{env, process};

use crate::exit_code;

/// An option represented by occurrence.
/// This option cannot appear more than once.
struct BooleanOption {
    value: bool,
    is_provided: bool,
}

impl BooleanOption {
    fn new(value: bool) -> Self {
        Self { value, is_provided: false }
    }
}

/// A struct containing program execution options.
pub struct Options {
    do_benchmark: BooleanOption,
}

impl Options {
    /// default `false`\
    /// (see [crate::benchmark])
    pub fn is_benchmark(&self) -> bool {
        self.do_benchmark.value
    }
}

impl Default for Options {
    /// Create a new [Self] with the default values.
    fn default() -> Self {
        Self { do_benchmark: BooleanOption::new(false) }
    }
}

/// Parses the command line arguments into an [Options].\
/// Exits the program with the exit code 1 if invalid arguments are provided.
/// (see [exit_code::INVALID_ARGUMENTS_ERROR])
pub fn parse_from_args() -> Options {
    let mut options = Options::default();

    for flag in env::args().skip(1) {
        match flag.to_lowercase().as_str() {
            "--benchmark" => {
                if options.do_benchmark.is_provided {
                    eprintln!("flag `--benchmark` supplied twice");
                    process::exit(exit_code::INVALID_ARGUMENTS_ERROR);
                }
                options.do_benchmark.value = true;
                options.do_benchmark.is_provided = true;
            }
            _ => {
                eprintln!("unexpected argument: {}", flag);
                process::exit(exit_code::INVALID_ARGUMENTS_ERROR)
            }
        };
    }
    options
}