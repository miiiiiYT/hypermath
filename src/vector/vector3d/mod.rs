use std::{fmt::Display, io::ErrorKind};

use rug::{ops::Pow, Float};

use crate::{point::Point3D, F64_PRECISION};

use super::Vector;

mod ops;

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Vector3D {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    _priv: (),
}

impl Vector3D {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z, _priv: () }
    }

    pub fn from_f64(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: Float::with_val(F64_PRECISION, x),
            y: Float::with_val(F64_PRECISION, y),
            z: Float::with_val(F64_PRECISION, z),
            _priv: ()
        }
    }

    /// Creates a new `Vector2D` from a `Vec`, truncating excess values.
    pub fn from_vec(vec: Vec<f64>) -> Result<Self, ErrorKind> {
        if vec.len() < 3 {
            return Err(ErrorKind::InvalidInput)
        }

        Ok(Self {
            x: Float::with_val(F64_PRECISION, vec[0]),
            y: Float::with_val(F64_PRECISION, vec[1]),
            z: Float::with_val(F64_PRECISION, vec[2]),
            _priv: ()
        })
    }

    pub fn from_points(point: Point3D, other: Point3D) -> Self {
        Self::new(
            other.x - point.x,
            other.y - point.y,
            other.z - point.z,
        )
    }
}

impl Vector for Vector3D {
    fn length(&self) -> Float {
        (self.x.clone().pow(2_i32) + self.y.clone().pow(2_i32) + self.z.clone().pow(2_i32)).sqrt()
    }

    fn as_vec(&self) -> Vec<Float> {
        vec![self.x.clone(), self.y.clone(), self.z.clone()]
    }
}

impl Display for Vector3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Default for Vector3D {
    fn default() -> Self {
        Self::from_f64(0.0, 0.0, 0.0)
    }
}

impl Eq for Vector3D {}