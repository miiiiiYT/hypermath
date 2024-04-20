use std::fmt::Display;

use rug::Float;

use crate::{vector::Vector2D, F64_PRECISION};

mod ops;

#[derive(Debug, PartialEq)]
pub struct Point2D {
    pub x: Float,
    pub y: Float,
    _priv: (),
}

impl Point2D {
    pub fn new(x: Float, y: Float) -> Self {
        Self { x, y, _priv: () }
    }

    pub fn from_f64(x: f64, y: f64) -> Self {
        Self {
            x: Float::with_val(F64_PRECISION, x),
            y: Float::with_val(F64_PRECISION, y),
            _priv: ()
        }
    }

    pub fn create_vector2d(point: Self, other: Self) -> Vector2D {
        Vector2D::new(
            other.x - point.x,
            other.y - point.y,
        )
    }
}

impl Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Eq for Point2D {}