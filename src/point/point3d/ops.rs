use rug::{ops::Pow, Float};

use crate::{point::{Point, ToPositionVector}, vector::Vector3D, F64_PRECISION};

use super::Point3D;

impl Point for Point3D {
    fn distance(&self, other: &Self) -> Float {
        Float::with_val(F64_PRECISION,
        ((other.x.clone() - self.x.clone()).pow(2_i32)
            + (other.y.clone() - self.x.clone()).pow(2_i32)
            + (other.z.clone() - self.z.clone()).pow(2_i32))
            .sqrt()
        )
    }

    fn midpoint(&self, other: &Self) -> Self {
        Self::new(
            (self.x.clone() + other.x.clone()) / 2,
            (self.y.clone() + other.y.clone()) / 2,
            (self.z.clone() + other.z.clone()) / 2,
        )
    }

    fn translate(&self, amt: Vec<Float>) -> Option<Self> {
        let mut vec = amt.iter();

        Some(Self::new(
            self.x.clone() + vec.next()?,
            self.y.clone() + vec.next()?,
            self.z.clone() + vec.next()?,
        ))
    }

    fn translate_mut(&mut self, amt: Vec<Float>) -> bool {
        if amt.len() < 3 {
            return false
        }

        let mut vec = amt.iter();

        self.x += vec.next().unwrap();
        self.y += vec.next().unwrap();
        self.z += vec.next().unwrap();
        return true
    }

    fn scale(&self, scale: Float) -> Self {
        Self::new(self.x.clone() * scale.clone(), self.y.clone() * scale.clone(), self.z.clone() * scale)
    }

    fn scale_mut(&mut self, scale: Float) {
        self.x *= scale.clone();
        self.y *= scale.clone();
        self.z *= scale;
    }

    fn truncate(&self, accuracy: usize) -> String {
        format!("({x:.prec$}, {y:.prec$}, {z:.prec$})", x = self.x, y = self.y, z = self.z, prec = accuracy)
    }
}

impl ToPositionVector for Point3D {
    type Return = Vector3D;

    fn to_position_vector(&self) -> Self::Return {
        Vector3D::new(self.x.clone(), self.y.clone(), self.z.clone())
    }
}