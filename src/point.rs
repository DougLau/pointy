// point.rs     2D Points
//
// Copyright (c) 2020-2025  Douglas P Lau
//
use crate::float::Float;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// 2-dimensional point / vector
///
/// ```rust
/// use pointy::Pt;
///
/// let pt = Pt::new(10.0, 15.0);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pt<F>
where
    F: Float,
{
    /// X coordinate
    pub x: F,

    /// Y coordinate
    pub y: F,
}

impl<F> From<&Pt<F>> for Pt<F>
where
    F: Float,
{
    fn from(pt: &Pt<F>) -> Self {
        Self { x: pt.x, y: pt.y }
    }
}

impl<F> From<(F, F)> for Pt<F>
where
    F: Float,
{
    fn from(pt: (F, F)) -> Self {
        Self { x: pt.0, y: pt.1 }
    }
}

impl<F> From<[F; 2]> for Pt<F>
where
    F: Float,
{
    fn from(pt: [F; 2]) -> Self {
        Self { x: pt[0], y: pt[1] }
    }
}

impl<F> Add for Pt<F>
where
    F: Float,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<F> Add<(F, F)> for Pt<F>
where
    F: Float,
{
    type Output = Self;

    fn add(self, rhs: (F, F)) -> Self {
        self + Self::from(rhs)
    }
}

impl<F> Sub for Pt<F>
where
    F: Float,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<F> Sub<(F, F)> for Pt<F>
where
    F: Float,
{
    type Output = Self;

    fn sub(self, rhs: (F, F)) -> Self {
        self - Self::from(rhs)
    }
}

impl<F> Mul<F> for Pt<F>
where
    F: Float,
{
    type Output = Self;

    fn mul(self, s: F) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

impl<F> Mul for Pt<F>
where
    F: Float,
{
    type Output = F;

    /// Get cross product with another vector.
    ///
    /// Returns the signed magnitude of the 3D cross product.
    fn mul(self, rhs: Self) -> F {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl<F> Mul<(F, F)> for Pt<F>
where
    F: Float,
{
    type Output = F;

    /// Get cross product with another vector.
    ///
    /// Returns the signed magnitude of the 3D cross product.
    fn mul(self, rhs: (F, F)) -> F {
        self * Self::from(rhs)
    }
}

impl<F> Div<F> for Pt<F>
where
    F: Float,
{
    type Output = Self;

    fn div(self, s: F) -> Self {
        Self {
            x: self.x / s,
            y: self.y / s,
        }
    }
}

impl<F> Neg for Pt<F>
where
    F: Float,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<F> Pt<F>
where
    F: Float,
{
    /// Create a new point
    pub fn new(x: F, y: F) -> Self {
        Self { x, y }
    }

    /// Create a unit vector from an angle (radians)
    pub fn from_angle(angle: F) -> Self {
        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    /// Create a point with minimum component values of two points
    pub fn with_min<P: Into<Self>>(self, rhs: P) -> Self {
        let rhs = rhs.into();
        let x = self.x.min(rhs.x);
        let y = self.y.min(rhs.y);
        Self { x, y }
    }

    /// Create a point with maximum component values of two points
    pub fn with_max<P: Into<Self>>(self, rhs: P) -> Self {
        let rhs = rhs.into();
        let x = self.x.max(rhs.x);
        let y = self.y.max(rhs.y);
        Self { x, y }
    }

    /// Get the magnitude (length) of a vector
    pub fn mag(self) -> F {
        self.x.hypot(self.y)
    }

    /// Normalize to unit length vector
    pub fn normalize(self) -> Self {
        let m = self.mag();
        if m > F::zero() {
            self / m
        } else {
            Self::default()
        }
    }

    /// Get distance squared to another point
    pub fn distance_sq<P: Into<Self>>(self, rhs: P) -> F {
        let v = self - rhs.into();
        v.x * v.x + v.y * v.y
    }

    /// Get distance to another point
    pub fn distance<P: Into<Self>>(self, rhs: P) -> F {
        (self - rhs.into()).mag()
    }

    /// Get the midpoint from this to another point
    pub fn midpoint<P: Into<Self>>(self, rhs: P) -> Self {
        let two = F::one() + F::one();
        let rhs = rhs.into();
        let x = (self.x + rhs.x) / two;
        let y = (self.y + rhs.y) / two;
        Self { x, y }
    }

    /// Calculate linear interpolation to another point.
    ///
    /// * `t` Interpolation amount, from 0 to 1
    pub fn lerp<P: Into<Self>>(self, rhs: P, t: F) -> Self {
        let rhs = rhs.into();
        let x = self.x.lerp(rhs.x, t);
        let y = self.y.lerp(rhs.y, t);
        Self { x, y }
    }

    /// Get left-hand perpendicular vector
    pub fn left(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// Get right-hand perpendicular vector
    pub fn right(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    /// Get dot product with another vector
    pub fn dot<P: Into<Self>>(self, rhs: P) -> F {
        let rhs = rhs.into();
        self.x * rhs.x + self.y * rhs.y
    }

    /// Get vector angle in radians
    pub fn angle(self) -> F {
        self.y.atan2(self.x)
    }

    /// Get relative angle to another vector.
    ///
    /// The result will be between `-PI` and `+PI`.
    pub fn angle_rel<P: Into<Self>>(self, rhs: P) -> F {
        let rhs = rhs.into();
        let th = self.angle() - rhs.angle();
        if th < -F::PI() {
            th + F::TAU()
        } else if th > F::PI() {
            th - F::TAU()
        } else {
            th
        }
    }
}

impl From<Pt<f32>> for Pt<f64> {
    fn from(pt: Pt<f32>) -> Self {
        Self {
            x: pt.x.into(),
            y: pt.y.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_approx_eq::*;

    #[test]
    fn points() {
        let a = Pt::new(2.0f32, 1.0);
        let b = Pt::new(3.0, 4.0);
        assert_eq!(a + b, Pt::new(5.0, 5.0));
        assert_eq!(b - a, Pt::new(1.0, 3.0));
        assert_eq!(a * 2.0, Pt::new(4.0, 2.0));
        assert_eq!(a / 2.0, Pt::new(1.0, 0.5));
        assert_eq!(-a, Pt::new(-2.0, -1.0));
        assert_eq!(b.mag(), 5.0);
        assert_eq!(a.normalize(), Pt::new(0.8944272, 0.4472136));
        assert_eq!(a.distance_sq(b), 10.0);
        assert_eq!(b.distance((0.0, 0.0)), 5.0);
        assert_eq!(a.midpoint(b), Pt::new(2.5, 2.5));
        assert_eq!(a.left(), Pt::new(-1.0, 2.0));
        assert_eq!(a.right(), Pt::new(1.0, -2.0));
    }

    #[test]
    fn angles() {
        let a = Pt::new(2.0f32, 1.0);
        let b = Pt::new(3.0, 4.0);
        let c = Pt::new(-1.0, 1.0);
        assert_eq!(Pt::new(0.0, 0.0).angle(), 0.0);
        assert_eq!(Pt::new(-1.0, 0.0).angle(), std::f32::consts::PI);
        assert_eq!(a.angle_rel(b), -0.4636476);
        assert_eq!(c.angle_rel((1.0, 1.0)), 1.5707963f32);
        assert_eq!(Pt::new(-1.0f32, -1.0).angle_rel(c), 1.5707965);
        let v = Pt::from_angle(0.0f32);
        assert_approx_eq!(v.x, 1.0);
        assert_approx_eq!(v.y, 0.0);
        let v = Pt::from_angle(std::f32::consts::PI / 2.0);
        assert_approx_eq!(v.x, 0.0);
        assert_approx_eq!(v.y, 1.0);
        let v = Pt::from_angle(std::f32::consts::PI);
        assert_approx_eq!(v.x, -1.0);
        assert_approx_eq!(v.y, 0.0);
        let v = Pt::from_angle(std::f32::consts::PI * 1.5);
        assert_approx_eq!(v.x, 0.0);
        assert_approx_eq!(v.y, -1.0);
    }
}
