mod error;

use std::char;
use std::i32;
use std::io::{self, BufRead};
use std::str::FromStr;

use error::{Error, Result};

/// Divides `num` by `denom`, returning an error if `denom` is zero.
pub fn safe_divide(num: i32, denom: i32) -> Result<i32> {
    if denom == 0 {
        return Err(Error::DivideByZero(num));
    }

    return Ok(num / denom);
}

/// Reads two integers from standard input and safely divides them.
pub fn safe_divide_from_stdin() -> Result<i32> {
    let stdin = io::stdin();
    let mut buf = String::new();

    let _ = stdin.lock().read_line(&mut buf)?;
    let mut split = buf.split(char::is_whitespace);

    let num = match split.next() {
        Some(s) => i32::from_str(s)?,
        None => return Err(Error::InvalidInput(buf.clone())),
    };

    let denom = match split.next() {
        Some(s) => i32::from_str(s)?,
        None => return Err(Error::InvalidInput(buf.clone())),
    };

    safe_divide(num, denom)
}
