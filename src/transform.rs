// transform.rs     Affine transforms
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::point::{Pt32, Pt64};
use std::ops::{Mul, MulAssign};

/// An affine transform for [Pt32] values.
///
/// A series of translate, rotate, scale and skew transformations can be
/// combined into a single `Transform32`.
///
/// [Pt32]: struct.Pt32.html
///
/// # Example
/// ```
/// use pointy::{Pt32, Transform32};
///
/// let t = Transform32::with_translate(-50.0, -50.0)
///     .rotate(std::f32::consts::PI)
///     .translate(50.0, 50.0)
///     .scale(2.0, 2.0);
/// let pt = Pt32(13.0, 5.5) * t;
/// let pt2 = (8.2, 4.7) * t;
/// let pt3 = t * (3.8, 9.6);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform32 {
    /// First six values in 3x3 matrix (last row assumed to be 0 0 1)
    e: [f32; 6],
}

/// An affine transform for [Pt64] values.
///
/// A series of translate, rotate, scale and skew transformations can be
/// combined into a single `Transform64`.
///
/// [Pt64]: struct.Pt64.html
///
/// # Example
/// ```
/// use pointy::{Pt64, Transform64};
///
/// let t = Transform64::with_translate(-50.0, -50.0)
///     .rotate(std::f64::consts::PI)
///     .translate(50.0, 50.0)
///     .scale(2.0, 2.0);
/// let pt = Pt64(13.0, 5.5) * t;
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform64 {
    /// First six values in 3x3 matrix (last row assumed to be 0 0 1)
    e: [f64; 6],
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

        impl Mul<($fty, $fty)> for $xty {
            type Output = $ptty;

            fn mul(self, s: ($fty, $fty)) -> $ptty {
                self * <$ptty>::from(s)
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

        impl Mul<$xty> for ($fty, $fty) {
            type Output = $ptty;

            fn mul(self, t: $xty) -> $ptty {
                <$ptty>::from(self) * t
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
    };
}

define_xform!(Transform32, Pt32, Pt32, f32);
define_xform!(Transform64, Pt64, Pt64, f64);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_identity() {
        assert_eq!(Transform32::default().e, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        assert_eq!(
            (Transform32::default() * Transform32::default()).e,
            [1.0, 0.0, 0.0, 0.0, 1.0, 0.0]
        );
        assert_eq!(Transform32::default() * Pt32(1.0, 2.0), Pt32(1.0, 2.0));
    }

    #[test]
    fn test_translate() {
        assert_eq!(
            Transform32::with_translate(1.5, -1.5).e,
            [1.0, 0.0, 1.5, 0.0, 1.0, -1.5]
        );
        assert_eq!(
            Transform32::default().translate(2.5, -3.5).e,
            [1.0, 0.0, 2.5, 0.0, 1.0, -3.5]
        );
        assert_eq!(
            Transform32::default().translate(5.0, 7.0) * Pt32(1.0, -2.0),
            Pt32(6.0, 5.0)
        );
    }

    #[test]
    fn test_scale() {
        assert_eq!(
            Transform32::with_scale(2.0, 4.0).e,
            [2.0, 0.0, 0.0, 0.0, 4.0, 0.0]
        );
        assert_eq!(
            Transform32::default().scale(3.0, 5.0).e,
            [3.0, 0.0, 0.0, 0.0, 5.0, 0.0]
        );
        assert_eq!(
            Transform32::default().scale(2.0, 3.0) * Pt32(1.5, -2.0),
            Pt32(3.0, -6.0)
        );
    }

    #[test]
    fn test_rotate() {
        const PI: f32 = std::f32::consts::PI;
        const V: f32 = 0.00000008742278;
        assert_eq!(
            Transform32::with_rotate(PI).e,
            [-1.0, V, 0.0, -V, -1.0, 0.0]
        );
        assert_eq!(
            Transform32::default().rotate(PI).e,
            [-1.0, V, 0.0, -V, -1.0, 0.0]
        );
        assert_eq!(
            Transform32::default().rotate(PI / 2.0) * Pt32(15.0, 7.0),
            Pt32(-7.0000005, 15.0)
        );
    }

    #[test]
    fn test_skew() {
        const PI: f32 = std::f32::consts::PI;
        assert_eq!(
            Transform32::with_skew(PI / 2.0, 0.0).e,
            [1.0, -22877334.0, 0.0, 0.0, 1.0, 0.0]
        );
        assert_eq!(
            Transform32::default().skew(PI / 2.0, 0.0).e,
            [1.0, -22877334.0, 0.0, 0.0, 1.0, 0.0]
        );
        assert_eq!(
            Transform32::with_skew(0.0, PI / 4.0).e,
            [1.0, 0.0, 0.0, 1.0, 1.0, 0.0]
        );
        assert_eq!(
            Transform32::default().skew(0.0, PI / 4.0).e,
            [1.0, 0.0, 0.0, 1.0, 1.0, 0.0]
        );
        assert_eq!(
            Transform32::default().skew(0.0, PI / 4.0) * (5.0, 3.0),
            Pt32(5.0, 8.0)
        );
        assert_eq!(
            Transform32::default().skew(0.0, PI / 4.0) * Pt32(15.0, 7.0),
            Pt32(15.0, 22.0)
        );
    }

    #[test]
    fn test_transform() {
        assert_eq!(
            (Transform32::with_translate(1.0, 2.0)
                * Transform32::with_scale(2.0, 2.0))
            .e,
            [2.0, 0.0, 2.0, 0.0, 2.0, 4.0]
        );
        assert_eq!(
            Transform32::with_translate(3.0, 5.0)
                * Transform32::with_scale(7.0, 11.0)
                * Transform32::with_rotate(std::f32::consts::PI / 2.0)
                * Transform32::with_skew(1.0, -2.0),
            Transform32::default()
                .translate(3.0, 5.0)
                .scale(7.0, 11.0)
                .rotate(std::f32::consts::PI / 2.0)
                .skew(1.0, -2.0)
        );
    }
}
