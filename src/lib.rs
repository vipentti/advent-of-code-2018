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

pub fn read_input_untrimmed() -> Result<String> {
    let mut s = String::new();

    let exe_name = env::current_exe()?;

    let name =
        exe_name.file_stem().ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Unable to get file_stem".to_owned()).into()
        })?;

    let mut file = File::open(format!("input/{}.txt", name.to_string_lossy()))?;

    file.read_to_string(&mut s)?;

    Ok(s)
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

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Self {
        Vector2 {
            x,
            y,
        }
    }

    /// [up, right, down, left]
    pub fn around(&self) -> [Vector2; 4] {
        [
            self.up(),
            self.right(),
            self.down(),
            self.left(),
        ]
    }

    pub fn up(&self) -> Self {
        *self + (0, -1)
    }

    pub fn down(&self) -> Self {
        *self + (0, 1)
    }

    pub fn left(&self) -> Self {
        *self + (-1, 0)
    }

    pub fn right(&self) -> Self {
        *self + (1, 0)
    }
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Vector2 {
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

impl PartialEq<(usize, usize)> for Vector2 {
    fn eq(&self, other: &(usize, usize)) -> bool {
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

impl std::convert::From<(usize, usize)> for Vector2 {
    fn from(v: (usize, usize)) -> Self {
        Vector2 { x: v.0 as i32, y: v.1 as i32 }
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

impl Add<(i32, i32)> for Vector2 {
    type Output = Vector2;

    fn add(self, other: (i32, i32)) -> Self::Output {
        Vector2 {
            x: self.x + other.0,
            y: self.y + other.1,
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

impl Sub<(i32, i32)> for Vector2 {
    type Output = Vector2;

    fn sub(self, other: (i32, i32)) -> Self::Output {
        Vector2 {
            x: self.x - other.0,
            y: self.y - other.1,
        }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl SubAssign<(i32, i32)> for Vector2 {
    fn sub_assign(&mut self, other: (i32, i32)) {
        *self = *self - other;
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl AddAssign<(i32, i32)> for Vector2 {
    fn add_assign(&mut self, other: (i32, i32)) {
        *self = *self + other;
    }
}


pub trait ToIndex {
    fn to_index(self, width: usize) -> usize;
}

impl ToIndex for Vector2 {
    fn to_index(self, width: usize) -> usize {
        if self.y < 0 {
            return usize::max_value();
        }
        if self.x < 0 {
            return usize::max_value();
        }
        if self.x > width as i32 - 1 {
            return usize::max_value();
        }

        self.y as usize * width + self.x as usize
    }
}

impl ToIndex for (i32, i32) {
    fn to_index(self, width: usize) -> usize {
        if self.1 < 0 || self.0 < 0 {
            return usize::max_value();
        }
        if self.0 > width as i32 - 1 {
            return usize::max_value();
        }

        self.1 as usize * width + self.0 as usize
    }
}

impl ToIndex for (usize, usize) {
    fn to_index(self, width: usize) -> usize {
        if self.0 > width - 1 {
            return usize::max_value();
        }
        self.1 * width + self.0
    }
}