mod vector2d;
mod vector3d;

use std::{fmt::Display, ops::{Add, AddAssign, Sub, SubAssign}};

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
    + PartialEq
    + PartialOrd
    + Clone
    + Display
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