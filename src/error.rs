use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::num::ParseIntError;

// ---------- Type declarations ----------

/// Alias for custom Result in our crate. This saves us from having to specify the error type
/// everywhere.
pub type Result<T> = ::std::result::Result<T, Error>;

/// Error type for our crate.
#[derive(Debug)]
pub enum Error {
    DivideByZero(i32),
    InvalidInput(String),
    Io(IoError),
    ParseError(ParseIntError),
}

// ---------- Type coercions ----------

/// Coercion from std::io::Error to our error type.
impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

/// Coercion from std::io::Error to our error type.
impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::ParseError(err)
    }
}

// ---------- Trait implementations ----------

/// Implements std::fmt::Display for our Error type. This is required to be able to implement
/// std::error::Error.
impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::DivideByZero(i) => write!(fmt, "cannot divide {} by 0", i),
            Error::InvalidInput(ref s) => write!(fmt, "input '{}' contains non-integers", s),
            Error::Io(ref err) => err.fmt(fmt),
            Error::ParseError(ref err) => err.fmt(fmt),
        }
    }
}

/// Implements std::error::Error for our Error type.
impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::DivideByZero(_) => "cannot divide by 0",
            Error::InvalidInput(_) => "insufficient number of inputs provided",
            Error::Io(ref err) => err.description(),
            Error::ParseError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::DivideByZero(_) |
            Error::InvalidInput(_) => None,
            Error::Io(ref err) => Some(err),
            Error::ParseError(ref err) => Some(err),
        }
    }
}
