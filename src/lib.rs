use crate::err::PrintError;

pub mod err;
pub mod exit_code;
pub mod options;
pub mod stdout;

#[cfg(feature = "time")]
pub mod time;

/// The Hello World string.
pub const HELLO_WORLD_STRING: &str = "Hello, world!";

/// A utility method for printing the [self::HELLO_WORLD_STRING] line to the standard output.
/// It is not advised to call this method more than once at a time.
/// Instead, create a new [StdoutWriter] and then call [StdoutWriter::println] without concern.
pub fn print_hello_world() -> Result<(), PrintError> {
    stdout::println(self::HELLO_WORLD_STRING)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::time;

    #[test]
    fn benchmark_display_test() {
        assert_eq!("1ns".to_string(), time::format_duration(&Duration::from_nanos(1)));
        assert_eq!("1μs".to_string(), time::format_duration(&Duration::from_micros(1)));
        assert_eq!("1ms".to_string(), time::format_duration(&Duration::from_millis(1)));
        assert_eq!("1s".to_string(), time::format_duration(&Duration::from_secs(1)));
    }
}