use regex;
use std::env;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

pub fn get_value<'a, T: std::str::FromStr>(
    caps: &regex::Captures<'a>,
    index: usize,
) -> std::result::Result<T, CustomError> {
    caps.get(index)
        .and_then(|v| v.as_str().parse::<T>().ok())
        .ok_or_else::<CustomError, _>(|| {
            CustomError(format!("Invalid {}", index))
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomError(pub String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

// This is important for other errors to wrap this one.
impl error::Error for CustomError {
    fn description(&self) -> &str {
        &self.0
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

pub type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

/// Reads the input from input/<day>.txt
/// Trims excess whitespace
pub fn read_input() -> Result<String> {
    let mut s = String::new();

    let exe_name = env::current_exe()?;

    let name =
        exe_name.file_stem().ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Unable to get file_stem".to_owned()).into()
        })?;

    let mut file = File::open(format!("input/{}.txt", name.to_string_lossy()))?;

    file.read_to_string(&mut s)?;

    Ok(s.trim().to_owned())
}
