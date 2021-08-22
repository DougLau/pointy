// float.rs
//
// Copyright (c) 2021  Douglas P Lau
//
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Floating point component type
pub trait Float:
    num_traits::Float
    + num_traits::FloatConst
    + Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
    + Sub<Output = Self>
    + Debug
    + Default
    + Copy
    + Clone
    + Sized
{
    /// Calculate linear interpolation of two values
    ///
    /// The t value should be between 0 and 1.
    fn lerp(self, rhs: Self, t: Self) -> Self {
        rhs + (self - rhs) * t
    }
}

impl Float for f32 {}
impl Float for f64 {}
