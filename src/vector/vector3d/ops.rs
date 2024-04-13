use std::ops::{Add, AddAssign, Sub, SubAssign};

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