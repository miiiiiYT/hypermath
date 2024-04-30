use std::fmt::{Debug, Display};

use rug::Float;

mod line3d;

pub use line3d::Line3D;

pub trait Line: Sized
    + Collapse
    + Clone
    + Debug
    + Display
    + Default
    + PartialEq
{
    /// Returns the magnitude, or the absolute value of the directional vector.
    fn magnitude(&self) -> Float;
}

pub trait Collapse {
    /// This should be in the same dimension as the Line, gets returned after collapsing
    type Output;

    /// Takes a `Float` as a lambda, which gets plugged into
    /// the general formula for lines in space. Returns `Self::Output`.  
    /// Note: Collapse is probably a shitty name, but it takes the line,
    /// plugs a value into it, and creates a point from that. Simple?
    fn collapse(&self, lambda: Float) -> Self::Output;
}