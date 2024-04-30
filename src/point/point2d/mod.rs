use std::fmt::Display;

use rug::Float;

use crate::F64_PRECISION;

mod ops;

/// A struct describing a geometrical Point in 2D space.
/// # Examples
/// ```rust
/// use hypermath::point::Point2D;
/// use hypermath::prelude::*;
/// 
/// // `Point2D::from_f64()` is used here instead of `new()`, as `new()` takes `rug::Float` as arguments.
/// // To make your code more readable, please use `from_f64()` where you're able to.
/// let point: Point2D = Point2D::from_f64(3.141, 1.41423);
/// println!("{}", point);
/// ```
/// Prints `(3.1410000000000000, 1.1414230000000001)`
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
}

impl Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Eq for Point2D {}