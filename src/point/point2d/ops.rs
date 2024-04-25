use rug::{ops::Pow, Float};

use crate::{point::Point, F64_PRECISION};

use super::Point2D;

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

    fn translate(&self, amt: Vec<Float>) -> Option<Self> {
        let mut vec = amt.iter();

        Some(Self::new(
            self.x.clone() + vec.next()?,
            self.y.clone() + vec.next()?,
        ))
    }

    fn translate_mut(&mut self, amt: Vec<Float>) -> bool {
        if amt.len() < 2 {
            return false
        }
        let mut vec = amt.iter();

        self.x += vec.next().unwrap();
        self.y += vec.next().unwrap();
        return true
    }

    fn scale(&self, scale: Float) -> Self {
        Self::new(self.x.clone() * scale.clone(), self.y.clone() * scale)
    }

    fn scale_mut(&mut self, scale: Float) {
        self.x *= scale.clone();
        self.y *= scale;
    }

    fn to_string(&self, accuracy: usize) -> String {
        format!("({x:.prec$}, {y:.prec$})", x = self.x, y = self.y, prec = accuracy)
    }
}