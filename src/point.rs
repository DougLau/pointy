// point.rs     2D Points
//
// Copyright (c) 2020  Douglas P Lau
//
use std::ops::{Add, Div, Mul, Neg, Sub};

/// 2-dimensional vector / point with `f32` values.
///
/// ```rust
/// use pointy::Pt32;
///
/// let pt = Pt32(10.0, 15.0);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pt32(pub f32, pub f32);

/// 2-dimensional vector / point with `f64` values.
///
/// ```rust
/// use pointy::Pt64;
///
/// let pt = Pt64(10.0, 15.0);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pt64(pub f64, pub f64);

macro_rules! define_pt {
    ($ptty:ty, $fty:ty, $pi:expr) => {
        impl Add for $ptty {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self(self.x() + rhs.x(), self.y() + rhs.y())
            }
        }

        impl Sub for $ptty {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self(self.x() - rhs.x(), self.y() - rhs.y())
            }
        }

        impl Mul<$fty> for $ptty {
            type Output = Self;

            fn mul(self, s: $fty) -> Self {
                Self(self.x() * s, self.y() * s)
            }
        }

        impl Mul for $ptty {
            type Output = $fty;

            /// Calculate the cross product of two vectors
            fn mul(self, rhs: Self) -> $fty {
                self.x() * rhs.y() - self.y() * rhs.x()
            }
        }

        impl Div<$fty> for $ptty {
            type Output = Self;

            fn div(self, s: $fty) -> Self {
                Self(self.x() / s, self.y() / s)
            }
        }

        impl Neg for $ptty {
            type Output = Self;

            fn neg(self) -> Self {
                Self(-self.x(), -self.y())
            }
        }

        impl $ptty {
            /// Get the X value
            pub const fn x(self) -> $fty {
                self.0
            }

            /// Get the Y value
            pub const fn y(self) -> $fty {
                self.1
            }

            /// Get the magnitude (length) of a vector
            pub fn mag(self) -> $fty {
                self.x().hypot(self.y())
            }

            /// Create a copy normalized to unit length
            pub fn normalize(self) -> Self {
                let m = self.mag();
                if m > 0.0 {
                    self / m
                } else {
                    Self::default()
                }
            }

            /// Calculate the distance squared between two points
            pub fn dist_sq(self, rhs: Self) -> $fty {
                let dx = self.x() - rhs.x();
                let dy = self.y() - rhs.y();
                dx * dx + dy * dy
            }

            /// Calculate the distance between two points
            pub fn dist(self, rhs: Self) -> $fty {
                self.dist_sq(rhs).sqrt()
            }

            /// Get the midpoint of two points
            pub fn midpoint(self, rhs: Self) -> Self {
                let x = (self.x() + rhs.x()) / 2.0;
                let y = (self.y() + rhs.y()) / 2.0;
                Self(x, y)
            }

            /// Create a left-hand perpendicular vector
            pub fn left(self) -> Self {
                Self(-self.y(), self.x())
            }

            /// Create a right-hand perpendicular vector
            pub fn right(self) -> Self {
                Self(self.y(), -self.x())
            }

            /// Calculate linear interpolation of two points.
            ///
            /// * `t` Interpolation amount, from 0 to 1
            pub fn lerp(self, rhs: Self, t: $fty) -> Self {
                let x = float_lerp(self.x(), rhs.x(), t);
                let y = float_lerp(self.y(), rhs.y(), t);
                Self(x, y)
            }

            /// Calculate the relative angle to another vector / point.
            ///
            /// The result will be between `-PI` and `+PI`.
            pub fn angle_rel(self, rhs: Self) -> $fty {
                let th = self.y().atan2(self.x()) - rhs.y().atan2(rhs.x());
                if th < -$pi {
                    th + 2.0 * $pi
                } else if th > $pi {
                    th - 2.0 * $pi
                } else {
                    th
                }
            }
        }
    };
}

define_pt!(Pt32, f32, std::f32::consts::PI);
define_pt!(Pt64, f64, std::f64::consts::PI);

/// Calculate linear interpolation of two values
///
/// The t value should be between 0 and 1.
fn float_lerp<T>(a: T, b: T, t: T) -> T
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    b + (a - b) * t
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pt() {
        let a = Pt32(2.0, 1.0);
        let b = Pt32(3.0, 4.0);
        let c = Pt32(-1.0, 1.0);
        assert_eq!(a + b, Pt32(5.0, 5.0));
        assert_eq!(b - a, Pt32(1.0, 3.0));
        assert_eq!(a * 2.0, Pt32(4.0, 2.0));
        assert_eq!(a / 2.0, Pt32(1.0, 0.5));
        assert_eq!(-a, Pt32(-2.0, -1.0));
        assert_eq!(b.mag(), 5.0);
        assert_eq!(a.normalize(), Pt32(0.8944272, 0.4472136));
        assert_eq!(a.dist_sq(b), 10.0);
        assert_eq!(b.dist(Pt32(0.0, 0.0)), 5.0);
        assert_eq!(a.midpoint(b), Pt32(2.5, 2.5));
        assert_eq!(a.left(), Pt32(-1.0, 2.0));
        assert_eq!(a.right(), Pt32(1.0, -2.0));
        assert_eq!(a.angle_rel(b), -0.4636476);
        assert_eq!(c.angle_rel(Pt32(1.0, 1.0)), 1.5707963);
        assert_eq!(Pt32(-1.0, -1.0).angle_rel(c), 1.5707965);
    }
}
