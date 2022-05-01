//! A module used for benchmarking the performance of the program.

use std::time::Duration;

/// Format a duration.
/// # Example
/// ```
/// use hello_world::time;
/// use std::time::Duration;
///
/// assert_eq!("1ns".to_string(), time::format_duration(&Duration::from_nanos(1)));
/// assert_eq!("1μs".to_string(), time::format_duration(&Duration::from_micros(1)));
/// ```
pub fn format_duration(duration: &Duration) -> String {
    const SMALL_TIME_UNITS: [char; 3] = ['n', 'μ', 'm'];

    let mut time = duration.as_nanos();

    for i in 0..3 {
        if time < 1000 {
            return format!("{}{}s", time, SMALL_TIME_UNITS[i]);
        }
        time /= 1000;
    }
    format!("{}s", duration.as_secs_f64())
}