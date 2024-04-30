mod ops;

use std::fmt::Display;

use rug::Float;

use crate::F64_PRECISION;

/// A struct describing a geometrical Point in 3D space.
/// # Examples
/// ```rust
/// use hypermath::point::Point3D;
/// use hypermath::prelude::*;
/// 
/// // `Point3D::from_f64()` is used here instead of `new()`, as `new()` takes `rug::Float` as arguments.
/// // To make your code more readable, please use `from_f64()` where you're able to.
/// let point: Point3D = Point3D::from_f64(2.0, 1.0, -1.2);
/// println!("{}", point.truncate(2));
/// ```
/// Prints `(2.0, 1.0, -1.2)`
#[derive(Debug, PartialEq)]
pub struct Point3D {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    _priv: (),
}

impl Point3D {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z, _priv: () }
    }

    pub fn from_f64(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: Float::with_val(F64_PRECISION, x),
            y: Float::with_val(F64_PRECISION, y),
            z: Float::with_val(F64_PRECISION, z),
            _priv: (),
        }
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Eq for Point3D {}