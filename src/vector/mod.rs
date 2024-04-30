mod vector2d;
mod vector3d;

use std::{fmt::{Debug, Display}, ops::{Add, AddAssign, Sub, SubAssign}};

use rug::Float;
pub use vector2d::Vector2D;
pub use vector3d::Vector3D;

pub trait Vector: Sized
    + Add
    + AddAssign
    + Sub
    + SubAssign
    + DotProduct
    + CrossProduct
    + ToPoint
    + PartialEq
    + PartialOrd
    + Clone
    + Display
    + Debug
{
    fn length(&self) -> Float;
    fn as_vec(&self) -> Vec<Float>;
}

pub trait DotProduct<Rhs = Self> {
    type Output;

    fn dot_product(self, rhs: Rhs) -> Self::Output;
}

pub trait CrossProduct<Rhs = Self> {
    type Output;

    fn cross_product(self, rhs: Rhs) -> Self::Output;
}

pub trait ToPoint {
    /// The type of point `to_point` takes and returns. When `Self` is Vector3D, `Point` should be Point3D.
    type Point;

    /// Returns a new point, derived from applying the vector to `starting_point`.
    fn to_point(&self, starting_point: Self::Point) -> Self::Point;
}