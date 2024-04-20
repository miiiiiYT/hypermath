mod ops;

use std::fmt::Display;

use rug::Float;

use crate::{vector::Vector3D, F64_PRECISION};

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

    pub fn create_vector3d(point: Self, other: Self) -> Vector3D {
        Vector3D::new(
            other.x - point.x,
            other.y - point.y,
            other.z - point.z,
        )
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Eq for Point3D {}