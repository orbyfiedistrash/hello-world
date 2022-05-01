//! The main file for the hello-world project.

use std::process;
use std::time::Instant;

use hello_world::{exit_code, options, time};
use hello_world::err::{PrintError, PrintErrorKind};
use hello_world::stdout::StdoutWriter;

fn print_benchmark() -> Result<(), PrintError> {
    let mut writer = StdoutWriter::default();
    writer.println("[*] starting benchmark...")?;

    let start = Instant::now();
    writer.println(hello_world::HELLO_WORLD_STRING)?;
    let elapsed = start.elapsed();

    let formatted = time::format_duration(&elapsed);
    writer.println(format!("[*] benchmark result: {}", formatted))
}

/// The main function of the hello-world program.
/// See [exit_code] for the possible exit codes of this program.
fn main() {
    {
        let options = options::parse_from_args();

        if options.is_benchmark() {
            match self::print_benchmark() {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("[!] benchmark failed: {}", err.take_error());
                    return;
                }
            }
            return;
        }
    }

    process::exit(match hello_world::print_hello_world() {
        Ok(_) => exit_code::OKAY,

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