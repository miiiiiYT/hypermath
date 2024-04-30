mod point2d;
mod point3d;

use std::fmt::{Debug, Display};

pub use point2d::Point2D;
pub use point3d::Point3D;
use rug::Float;

/// A trait that may be implemented to represent a geometrical point.
/// 
/// It uses [`rug::Float`]s to store floating-point values, which implements
/// [the GNU MPFR library](https://www.mpfr.org/)
/// to accurately store and treat floating-point numbers. See[the wikipedia artice on rounding errors](https://en.wikipedia.org/wiki/Round-off_error)
/// for more information on why MPFR was chosen over the `f64` primitive.  
/// Note: The library often produces numbers accurate to many decimal places. You should probably determine the amount of precision you need
/// for yourself and then round the resulting numbers before displaying or priting them. 
/// 
/// See [`hypermath::point::Point2D`] or [`hypermath::point::Point3D`] for info on how to use this trait.
/// Ideally you just use these two structs, unless you want to do some crazy 4D stuff or whatever
pub trait Point: Sized
    + PartialEq
    + Eq
    + Display
    + Debug
    + Default
    + ToPositionVector
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

    /// Truncates the values of the point to the supplied accuracy and returns a string that can be displayed to the user.
    /// # Examples
    /// ```rust
    /// use hypermath::point::Point2D;
    /// use hypermath::prelude::*;
    /// 
    /// let point: Point2D = Point2D::from_f64(3.141, 1.141423);
    /// println!("{}", point);
    /// println!("{}", point.truncate(7));
    /// ```
    /// Prints:
    /// ```text
    /// (3.1410000000000000, 1.1414230000000001)
    /// (3.141000, 1.141423)
    /// ```
    fn truncate(&self, accuracy: usize) -> String;
}

pub trait ToPositionVector {
    /// The appropriate vector type. On a 2D point, this should be a 2D vector and so on.
    type Return;

    /// Turns `Self` into a position vector, returns the appropriate vector type.
    fn to_position_vector(&self) -> Self::Return;
}