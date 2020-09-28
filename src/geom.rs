// geom.rs    Simple geometry stuff.
//
// Copyright (c) 2020  Douglas P Lau
//
use std::ops::{Add, Div, Mul, MulAssign, Neg, Sub};

/// 2-dimensional vector / point with `f32` values.
///
/// ```rust
/// use pointy::Pt;
///
/// let pt = Pt(10.0, 15.0);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pt(pub f32, pub f32);

/// 2-dimensional vector / point with `f64` values.
///
/// ```rust
/// use pointy::PtB;
///
/// let pt = PtB(10.0, 15.0);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PtB(pub f64, pub f64);

/// An affine transform for [Pt] values.
///
/// A series of translate, rotate, scale and skew transformations can be
/// combined into a single `Transform`.
///
/// [Pt]: struct.Pt.html
///
/// # Example
/// ```
/// use pointy::{Pt, Transform};
///
/// let t = Transform::with_translate(-50.0, -50.0)
///     .rotate(std::f32::consts::PI)
///     .translate(50.0, 50.0)
///     .scale(2.0, 2.0);
/// let pt = Pt(13.0, 5.5) * t;
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    /// First six values in 3x3 matrix (last row assumed to be 0 0 1)
    e: [f32; 6],
}

/// An affine transform for [PtB] values.
///
/// A series of translate, rotate, scale and skew transformations can be
/// combined into a single `TransformB`.
///
/// [PtB]: struct.PtB.html
///
/// # Example
/// ```
/// use pointy::{PtB, TransformB};
///
/// let t = TransformB::with_translate(-50.0, -50.0)
///     .rotate(std::f64::consts::PI)
///     .translate(50.0, 50.0)
///     .scale(2.0, 2.0);
/// let pt = PtB(13.0, 5.5) * t;
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransformB {
    /// First six values in 3x3 matrix (last row assumed to be 0 0 1)
    e: [f64; 6],
}

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
    }
}

define_pt!(Pt, f32, std::f32::consts::PI);
define_pt!(PtB, f64, std::f64::consts::PI);

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

macro_rules! define_xform {
    ($xty:ty, $ptty:ty, $ptexp:expr, $fty:ty) => {
        impl MulAssign for $xty {
            fn mul_assign(&mut self, rhs: Self) {
                self.e = self.mul_e(&rhs);
            }
        }

        impl Mul for $xty {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self {
                let e = self.mul_e(&rhs);
                Self { e }
            }
        }

        impl Mul<$ptty> for $xty {
            type Output = $ptty;

            fn mul(self, s: $ptty) -> $ptty {
                let x = self.e[0] * s.x() + self.e[1] * s.y() + self.e[2];
                let y = self.e[3] * s.x() + self.e[4] * s.y() + self.e[5];
                $ptexp(x, y)
            }
        }

        impl Mul<$xty> for $ptty {
            type Output = $ptty;

            fn mul(self, t: $xty) -> $ptty {
                let x = t.e[0] * self.x() + t.e[1] * self.y() + t.e[2];
                let y = t.e[3] * self.x() + t.e[4] * self.y() + t.e[5];
                $ptexp(x, y)
            }
        }

        impl Default for $xty {
            /// Create a new identity transform.
            fn default() -> Self {
                Self {
                    e: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
                }
            }
        }

        impl $xty {
            /// Multiple two affine transforms.
            fn mul_e(&self, rhs: &Self) -> [$fty; 6] {
                let mut e = [0.0; 6];
                e[0] = self.e[0] * rhs.e[0] + self.e[3] * rhs.e[1];
                e[1] = self.e[1] * rhs.e[0] + self.e[4] * rhs.e[1];
                e[2] = self.e[2] * rhs.e[0] + self.e[5] * rhs.e[1] + rhs.e[2];
                e[3] = self.e[0] * rhs.e[3] + self.e[3] * rhs.e[4];
                e[4] = self.e[1] * rhs.e[3] + self.e[4] * rhs.e[4];
                e[5] = self.e[2] * rhs.e[3] + self.e[5] * rhs.e[4] + rhs.e[5];
                e
            }

            /// Create a new translation transform.
            ///
            /// * `tx` Amount to translate X.
            /// * `ty` Amount to translate Y.
            pub const fn with_translate(tx: $fty, ty: $fty) -> Self {
                Self {
                    e: [1.0, 0.0, tx, 0.0, 1.0, ty],
                }
            }

            /// Create a new scale transform.
            ///
            /// * `sx` Scale factor for X dimension.
            /// * `sy` Scale factor for Y dimension.
            pub const fn with_scale(sx: $fty, sy: $fty) -> Self {
                Self {
                    e: [sx, 0.0, 0.0, 0.0, sy, 0.0],
                }
            }

            /// Create a new rotation transform.
            ///
            /// * `th` Angle to rotate coordinates (radians).
            pub fn with_rotate(th: $fty) -> Self {
                let sn = th.sin();
                let cs = th.cos();
                Self {
                    e: [cs, -sn, 0.0, sn, cs, 0.0],
                }
            }

            /// Create a new skew transform.
            ///
            /// * `ax` Angle to skew X-axis (radians).
            /// * `ay` Angle to skew Y-axis (radians).
            pub fn with_skew(ax: $fty, ay: $fty) -> Self {
                let tnx = ax.tan();
                let tny = ay.tan();
                Self {
                    e: [1.0, tnx, 0.0, tny, 1.0, 0.0],
                }
            }

            /// Apply translation to a transform.
            ///
            /// * `tx` Amount to translate X.
            /// * `ty` Amount to translate Y.
            pub fn translate(mut self, tx: $fty, ty: $fty) -> Self {
                self *= Self::with_translate(tx, ty);
                self
            }

            /// Apply scaling to a transform.
            ///
            /// * `sx` Scale factor for X dimension.
            /// * `sy` Scale factor for Y dimension.
            pub fn scale(mut self, sx: $fty, sy: $fty) -> Self {
                self *= Self::with_scale(sx, sy);
                self
            }

            /// Apply rotation to a transform.
            ///
            /// * `th` Angle to rotate coordinates (radians).
            pub fn rotate(mut self, th: $fty) -> Self {
                self *= Self::with_rotate(th);
                self
            }

            /// Apply skew to a transform.
            ///
            /// * `ax` Angle to skew X-axis (radians).
            /// * `ay` Angle to skew Y-axis (radians).
            pub fn skew(mut self, ax: $fty, ay: $fty) -> Self {
                self *= Self::with_skew(ax, ay);
                self
            }
        }
    }
}

define_xform!(Transform, Pt, Pt, f32);
define_xform!(TransformB, PtB, PtB, f64);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pt() {
        let a = Pt(2.0, 1.0);
        let b = Pt(3.0, 4.0);
        let c = Pt(-1.0, 1.0);
        assert_eq!(a + b, Pt(5.0, 5.0));
        assert_eq!(b - a, Pt(1.0, 3.0));
        assert_eq!(a * 2.0, Pt(4.0, 2.0));
        assert_eq!(a / 2.0, Pt(1.0, 0.5));
        assert_eq!(-a, Pt(-2.0, -1.0));
        assert_eq!(b.mag(), 5.0);
        assert_eq!(a.normalize(), Pt(0.8944272, 0.4472136));
        assert_eq!(a.dist_sq(b), 10.0);
        assert_eq!(b.dist(Pt(0.0, 0.0)), 5.0);
        assert_eq!(a.midpoint(b), Pt(2.5, 2.5));
        assert_eq!(a.left(), Pt(-1.0, 2.0));
        assert_eq!(a.right(), Pt(1.0, -2.0));
        assert_eq!(a.angle_rel(b), -0.4636476);
        assert_eq!(c.angle_rel(Pt(1.0, 1.0)), 1.5707963);
        assert_eq!(Pt(-1.0, -1.0).angle_rel(c), 1.5707965);
    }

    #[test]
    fn test_identity() {
        assert_eq!(Transform::default().e, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        assert_eq!(
            (Transform::default() * Transform::default()).e,
            [1.0, 0.0, 0.0, 0.0, 1.0, 0.0]
        );
        assert_eq!(Transform::default() * Pt(1.0, 2.0), Pt(1.0, 2.0));
    }

    #[test]
    fn test_translate() {
        assert_eq!(
            Transform::with_translate(1.5, -1.5).e,
            [1.0, 0.0, 1.5, 0.0, 1.0, -1.5]
        );
        assert_eq!(
            Transform::default().translate(2.5, -3.5).e,
            [1.0, 0.0, 2.5, 0.0, 1.0, -3.5]
        );
        assert_eq!(
            Transform::default().translate(5.0, 7.0) * Pt(1.0, -2.0),
            Pt(6.0, 5.0)
        );
    }

    #[test]
    fn test_scale() {
        assert_eq!(
            Transform::with_scale(2.0, 4.0).e,
            [2.0, 0.0, 0.0, 0.0, 4.0, 0.0]
        );
        assert_eq!(
            Transform::default().scale(3.0, 5.0).e,
            [3.0, 0.0, 0.0, 0.0, 5.0, 0.0]
        );
        assert_eq!(
            Transform::default().scale(2.0, 3.0) * Pt(1.5, -2.0),
            Pt(3.0, -6.0)
        );
    }

    #[test]
    fn test_rotate() {
        const PI: f32 = std::f32::consts::PI;
        const V: f32 = 0.00000008742278;
        assert_eq!(Transform::with_rotate(PI).e, [-1.0, V, 0.0, -V, -1.0, 0.0]);
        assert_eq!(
            Transform::default().rotate(PI).e,
            [-1.0, V, 0.0, -V, -1.0, 0.0]
        );
        assert_eq!(
            Transform::default().rotate(PI / 2.0) * Pt(15.0, 7.0),
            Pt(-7.0000005, 15.0)
        );
    }

    #[test]
    fn test_skew() {
        const PI: f32 = std::f32::consts::PI;
        assert_eq!(
            Transform::with_skew(PI / 2.0, 0.0).e,
            [1.0, -22877334.0, 0.0, 0.0, 1.0, 0.0]
        );
        assert_eq!(
            Transform::default().skew(PI / 2.0, 0.0).e,
            [1.0, -22877334.0, 0.0, 0.0, 1.0, 0.0]
        );
        assert_eq!(
            Transform::with_skew(0.0, PI / 4.0).e,
            [1.0, 0.0, 0.0, 1.0, 1.0, 0.0]
        );
        assert_eq!(
            Transform::default().skew(0.0, PI / 4.0).e,
            [1.0, 0.0, 0.0, 1.0, 1.0, 0.0]
        );
        assert_eq!(
            Transform::default().skew(0.0, PI / 4.0) * Pt(5.0, 3.0),
            Pt(5.0, 8.0)
        );
        assert_eq!(
            Transform::default().skew(0.0, PI / 4.0) * Pt(15.0, 7.0),
            Pt(15.0, 22.0)
        );
    }

    #[test]
    fn test_transform() {
        assert_eq!(
            (Transform::with_translate(1.0, 2.0)
                * Transform::with_scale(2.0, 2.0))
            .e,
            [2.0, 0.0, 2.0, 0.0, 2.0, 4.0]
        );
        assert_eq!(
            Transform::with_translate(3.0, 5.0)
                * Transform::with_scale(7.0, 11.0)
                * Transform::with_rotate(std::f32::consts::PI / 2.0)
                * Transform::with_skew(1.0, -2.0),
            Transform::default()
                .translate(3.0, 5.0)
                .scale(7.0, 11.0)
                .rotate(std::f32::consts::PI / 2.0)
                .skew(1.0, -2.0)
        );
    }
}
