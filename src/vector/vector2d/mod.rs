use std::{fmt::Display, io::ErrorKind};

use rug::{ops::Pow, Float};

use crate::F64_PRECISION;

use super::Vector;

mod ops;

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Vector2D {
    pub x: Float,
    pub y: Float,
}

impl Vector2D {
    pub fn new(x: Float, y: Float) -> Self {
        Self { x, y }
    }

    pub fn from_f64(x: f64, y: f64) -> Self {
        Self {x: Float::with_val(F64_PRECISION, x), y: Float::with_val(F64_PRECISION, y) }
    }

    /// Creates a new `Vector2D` from a `Vec`, truncating excess values.
    pub fn from_vec(vec: Vec<f64>) -> Result<Self, ErrorKind> {
        if vec.len() < 2 {
            return Err(ErrorKind::InvalidInput)
        }

        Ok(Self { x: Float::with_val(F64_PRECISION, vec[0]), y: Float::with_val(F64_PRECISION, vec[1]) })
    }
}

impl Vector for Vector2D {
    fn length(&self) -> Float {
        (self.x.clone().pow(2_i32) + self.y.clone().pow(2_i32)).sqrt()
    }
}

impl Display for Vector2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(f64, f64)> for Vector2D {
    fn from(value: (f64, f64)) -> Self {
        self::Vector2D::from_f64(value.0, value.1)
    }
}

impl TryFrom<Vec<f64>> for Vector2D {
    type Error = ErrorKind;

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        self::Vector2D::from_vec(value)
    }
}