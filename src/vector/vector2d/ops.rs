use std::ops::{Add, AddAssign, Sub, SubAssign};

use rug::Float;

use crate::{point::Point2D, vector::{CrossProduct, DotProduct, ToPoint}};

use super::Vector2D;

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl DotProduct for Vector2D {
    type Output = Float;

    /// Returns the dot product of this vector and the right hand side vector, without mutating the original.
    /// 
    fn dot_product(self, rhs: Self) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        x + y
    }
}

impl CrossProduct for Vector2D {
    type Output = Float;

    fn cross_product(self, rhs: Self) -> Self::Output {
        (self.x * rhs.y) - (self.y * rhs.x)
    }
}

impl ToPoint for Vector2D {
    type Point = Point2D;

    fn to_point(&self, starting_point: Self::Point) -> Self::Point {
        Point2D::new(self.x.clone() + starting_point.x, self.y.clone() + starting_point.y)
    }
}