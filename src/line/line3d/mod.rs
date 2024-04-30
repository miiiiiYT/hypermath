use std::fmt::Display;

use rug::Float;

use crate::{point::Point3D, vector::{ToPoint, Vector, Vector3D}};

use super::{Collapse, Line};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Line3D {
    support_vector: Vector3D,
    directional_vector: Vector3D,
}

impl Line3D {
    pub fn new(support_vector: Vector3D, directional_vector: Vector3D) -> Self {
        Self { support_vector, directional_vector }
    }
}

impl Line for Line3D {
    fn magnitude(&self) -> Float {
        self.directional_vector.length()
    }
}

impl Display for Line3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{supVec} + r * {dirVec}", supVec=self.support_vector, dirVec=self.directional_vector)
    }
}

impl Collapse for Line3D {
    type Output = Point3D;

    fn collapse(&self, lambda: Float) -> Self::Output {
        (self.support_vector.clone() + (self.directional_vector.clone() * lambda)).to_point(Point3D::from_f64(0.0, 0.0, 0.0))
    }
}