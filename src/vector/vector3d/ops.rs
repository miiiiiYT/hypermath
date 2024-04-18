use std::ops::{Add, AddAssign, Sub, SubAssign};

use rug::Float;

use crate::vector::{CrossProduct, DotProduct};

use super::Vector3D;

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3D::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3D::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

impl SubAssign for Vector3D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl DotProduct for Vector3D {
    type Output = Float;

    fn dot_product(self, rhs: Self) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        x + y + z
    }
}

impl CrossProduct for Vector3D {
    type Output = Self;

    fn cross_product(self, rhs: Self) -> Self::Output {
        let x = (self.y.clone() * rhs.z.clone()) - (self.z.clone() * rhs.y.clone());
        let y = (self.z.clone() * rhs.x.clone()) - (self.x.clone() * rhs.z.clone());
        let z = (self.x.clone() * rhs.y.clone()) - (self.y.clone() * rhs.x.clone());
        Vector3D::new(x, y, z)
    }
}