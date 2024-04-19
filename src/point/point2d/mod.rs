use std::fmt::Display;

use rug::{ops::Pow, Float};

use crate::{vector::Vector2D, F64_PRECISION};

use super::Point;

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

impl Point for Point2D {
    fn distance(&self, other: &Self) -> Float {
        Float::with_val(F64_PRECISION,
            ((other.x.clone() - self.x.clone()).pow(2_i32) + (other.y.clone() - self.x.clone()).pow(2_i32)).sqrt()
        )
    }

    fn midpoint(&self, other: &Self) -> Self {
        Self::new(
            (self.x.clone() + other.x.clone()) / 2,
            (self.y.clone() + other.y.clone()) / 2
        )
    }

    fn translate(&self, amt: (Float, Float)) -> Self {
        Self::new(self.x.clone() + amt.0, self.y.clone() + amt.1)
    }

    fn translate_mut(&mut self, amt: (Float, Float)) {
        self.x += amt.0;
        self.y += amt.1;
    }

    fn scale(&self, scale: Float) -> Self {
        Self::new(self.x.clone() * scale.clone(), self.y.clone() * scale)
    }

    fn scale_mut(&mut self, scale: Float) {
        self.x *= scale.clone();
        self.y *= scale;
    }
}

impl Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Eq for Point2D {}