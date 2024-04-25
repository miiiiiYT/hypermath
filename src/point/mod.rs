mod point2d;
mod point3d;

use std::fmt::{Debug, Display};

pub use point2d::Point2D;
pub use point3d::Point3D;
use rug::Float;

pub trait Point: Sized
    + PartialEq
    + Eq
    + Display
    + Debug
{
    /// Returns the distance between `self` and `other` as a `Float`.
    fn distance(&self, other: &Self) -> Float;

    /// Returns a new `Point` that's exactly between `self` and `other`.
    fn midpoint(&self, other: &Self) -> Self;

    /// Returns a new `Point` translated by `amt` while keeping the old point.
    fn translate(&self, amt: Vec<Float>) -> Option<Self>;

    /// Translates `self` by `amt`, mutating the original.
    fn translate_mut(&mut self, amt: Vec<Float>) -> bool;

    /// Scales `self` by `scale`, returning a new `Point`.
    fn scale(&self, scale: Float) -> Self;

    /// Scales `self` by `scale`, mutating the original.
    fn scale_mut(&mut self, scale: Float);

    fn to_string(&self, accuracy: usize) -> String;
}