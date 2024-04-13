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
    + PartialEq
    + PartialOrd
    + Clone
    + Display
{
    fn length(&self) -> Float;
}