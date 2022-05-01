//! A module providing the exit codes for the program.

/// An IO error occurred while printing the message to the standard output.
pub const OPERATION_ERROR: i32 = -1;

/// No errors occurred while printing to the standard output.
pub const OKAY: i32 = 0;

/// One or more invalid arguments were supplied to the program.
pub const INVALID_ARGUMENTS_ERROR: i32 = 1;