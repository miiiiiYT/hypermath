use std::{fmt::Display, io::ErrorKind};

use rug::{ops::Pow, Float};

use crate::{point::Point2D, F64_PRECISION};

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

    pub fn from_points(point: Point2D, other: Point2D) -> Self {
        Self::new(
            other.x - point.x,
            other.y - point.y,
        )
    }
}

impl Vector for Vector2D {
    fn length(&self) -> Float {
        (self.x.clone().pow(2_i32) + self.y.clone().pow(2_i32)).sqrt()
    }

    fn as_vec(&self) -> Vec<Float> {
       vec![self.x.clone(), self.y.clone()] 
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

impl Default for Vector2D {
    fn default() -> Self {
        Self::from_f64(0.0, 0.0)
    }
}

impl Eq for Vector2D {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Ensure [`Vector2D::from_f64()`] behaves the same way as [`Vector2D::new()`].
    fn new_eq_from_f64() {
        assert_eq!(
            Vector2D::new(Float::with_val(F64_PRECISION, 2.4), Float::with_val(F64_PRECISION, -7.4)),
            Vector2D::from_f64(2.4, -7.4),
        )
    }

    #[test]
    /// Ensure [`Vector2D::from_vec()`] behaves the same way as [`Vector2D::new()`].
    fn new_eq_from_vec() {
        assert_eq!(
            Vector2D::new(Float::with_val(F64_PRECISION, 2.4), Float::with_val(F64_PRECISION, -7.4)),
            Vector2D::from_vec(vec![2.4, -7.4]).unwrap(),
        )
    }

    #[test]
    /// Ensure supplying too little values fails
    fn fail_on_too_little_values() {
        assert!(Vector2D::from_vec(vec![1.0]).is_err())
    }

    #[test]
    fn ensure_proper_value_storage() {
        assert_eq!(
            Vector2D::new(Float::with_val(F64_PRECISION, 2.3), Float::with_val(F64_PRECISION, -6.23)).x,
            Float::with_val(F64_PRECISION, 2.3)
        );
        assert_eq!(
            Vector2D::new(Float::with_val(F64_PRECISION, 2.3), Float::with_val(F64_PRECISION, -6.23)).y,
            Float::with_val(F64_PRECISION, -6.23)
        )
    }
}