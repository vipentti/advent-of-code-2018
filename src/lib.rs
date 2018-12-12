use regex;
use std::convert::From;
use std::env;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

pub fn get_value<'a, T: std::str::FromStr>(
    caps: &regex::Captures<'a>,
    index: usize,
) -> std::result::Result<T, CustomError> {
    caps.get(index)
        .and_then(|v| v.as_str().trim().parse::<T>().ok())
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

/// Reads the input from input/<day>.txt
pub fn read_file(file_name: &str) -> Result<String> {
    let mut s = String::new();

    let mut file = File::open(file_name)?;

    file.read_to_string(&mut s)?;

    Ok(s)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq<(i32, i32)> for Vector2 {
    fn eq(&self, other: &(i32, i32)) -> bool {
        let v: Vector2 = (*other).into();
        *self == v
    }
}

impl From<Vector2> for (i32, i32) {
    fn from(v: Vector2) -> Self {
        (v.x, v.y)
    }
}

impl From<(i32, i32)> for Vector2 {
    fn from(v: (i32, i32)) -> Self {
        Vector2 { x: v.0, y: v.1 }
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Self) -> Self::Output {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i32> for Vector2 {
    type Output = Vector2;

    fn mul(self, other: i32) -> Self {
        Vector2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Self) -> Self::Output {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
