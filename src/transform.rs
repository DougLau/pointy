// transform.rs     Affine transforms
//
// Copyright (c) 2020-2022  Douglas P Lau
//
use crate::float::Float;
use crate::point::Pt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Mul, MulAssign};

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
/// let pt = Pt::new(13.0, 5.5) * t;
/// let pt2 = (8.2, 4.7) * t;
/// let pt3 = t * (3.8, 9.6);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Transform<F>
where
    F: Float,
{
    /// First six values in 3x3 matrix (last row assumed to be 0 0 1)
    e: [F; 6],
}

impl<F> MulAssign for Transform<F>
where
    F: Float,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.e = self.mul_e(&rhs);
    }
}

impl<F> Mul for Transform<F>
where
    F: Float,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let e = self.mul_e(&rhs);
        Self { e }
    }
}

impl<F> Mul<Pt<F>> for Transform<F>
where
    F: Float,
{
    type Output = Pt<F>;

    fn mul(self, s: Pt<F>) -> Pt<F> {
        let x = self.e[0] * s.x() + self.e[1] * s.y() + self.e[2];
        let y = self.e[3] * s.x() + self.e[4] * s.y() + self.e[5];
        Pt::new(x, y)
    }
}

impl<F> Mul<(F, F)> for Transform<F>
where
    F: Float,
{
    type Output = Pt<F>;

    fn mul(self, s: (F, F)) -> Pt<F> {
        self * Pt::from(s)
    }
}

impl<F> Mul<Transform<F>> for Pt<F>
where
    F: Float,
{
    type Output = Pt<F>;

    fn mul(self, t: Transform<F>) -> Self {
        let x = t.e[0] * self.x() + t.e[1] * self.y() + t.e[2];
        let y = t.e[3] * self.x() + t.e[4] * self.y() + t.e[5];
        Pt::new(x, y)
    }
}

impl<F> Mul<Transform<F>> for (F, F)
where
    F: Float,
{
    type Output = Pt<F>;

    fn mul(self, t: Transform<F>) -> Pt<F> {
        Pt::from(self) * t
    }
}

impl<F> Default for Transform<F>
where
    F: Float,
{
    /// Create a new identity transform.
    fn default() -> Self {
        Self {
            e: [
                F::one(),
                F::zero(),
                F::zero(),
                F::zero(),
                F::one(),
                F::zero(),
            ],
        }
    }
}

impl<F> Transform<F>
where
    F: Float,
{
    /// Multiple two affine transforms.
    fn mul_e(&self, rhs: &Self) -> [F; 6] {
        [
            self.e[0] * rhs.e[0] + self.e[3] * rhs.e[1],
            self.e[1] * rhs.e[0] + self.e[4] * rhs.e[1],
            self.e[2] * rhs.e[0] + self.e[5] * rhs.e[1] + rhs.e[2],
            self.e[0] * rhs.e[3] + self.e[3] * rhs.e[4],
            self.e[1] * rhs.e[3] + self.e[4] * rhs.e[4],
            self.e[2] * rhs.e[3] + self.e[5] * rhs.e[4] + rhs.e[5],
        ]
    }

    /// Create a new translation transform.
    ///
    /// * `tx` Amount to translate X.
    /// * `ty` Amount to translate Y.
    pub fn with_translate(tx: F, ty: F) -> Self {
        Self {
            e: [F::one(), F::zero(), tx, F::zero(), F::one(), ty],
        }
    }

    /// Create a new scale transform.
    ///
    /// * `sx` Scale factor for X dimension.
    /// * `sy` Scale factor for Y dimension.
    pub fn with_scale(sx: F, sy: F) -> Self {
        Self {
            e: [sx, F::zero(), F::zero(), F::zero(), sy, F::zero()],
        }
    }

    /// Create a new rotation transform.
    ///
    /// * `th` Angle to rotate coordinates (radians).
    pub fn with_rotate(th: F) -> Self {
        let sn = th.sin();
        let cs = th.cos();
        Self {
            e: [cs, -sn, F::zero(), sn, cs, F::zero()],
        }
    }

    /// Create a new skew transform.
    ///
    /// * `ax` Angle to skew X-axis (radians).
    /// * `ay` Angle to skew Y-axis (radians).
    pub fn with_skew(ax: F, ay: F) -> Self {
        let tnx = ax.tan();
        let tny = ay.tan();
        Self {
            e: [F::one(), tnx, F::zero(), tny, F::one(), F::zero()],
        }
    }

    /// Apply translation to a transform.
    ///
    /// * `tx` Amount to translate X.
    /// * `ty` Amount to translate Y.
    pub fn translate(mut self, tx: F, ty: F) -> Self {
        self *= Self::with_translate(tx, ty);
        self
    }

    /// Apply scaling to a transform.
    ///
    /// * `sx` Scale factor for X dimension.
    /// * `sy` Scale factor for Y dimension.
    pub fn scale(mut self, sx: F, sy: F) -> Self {
        self *= Self::with_scale(sx, sy);
        self
    }

    /// Apply rotation to a transform.
    ///
    /// * `th` Angle to rotate coordinates (radians).
    pub fn rotate(mut self, th: F) -> Self {
        self *= Self::with_rotate(th);
        self
    }

    /// Apply skew to a transform.
    ///
    /// * `ax` Angle to skew X-axis (radians).
    /// * `ay` Angle to skew Y-axis (radians).
    pub fn skew(mut self, ax: F, ay: F) -> Self {
        self *= Self::with_skew(ax, ay);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_identity() {
        assert_eq!(
            Transform::<f32>::default().e,
            [1.0, 0.0, 0.0, 0.0, 1.0, 0.0]
        );
        assert_eq!(
            (Transform::<f64>::default() * Transform::default()).e,
            [1.0, 0.0, 0.0, 0.0, 1.0, 0.0]
        );
        assert_eq!(Transform::default() * Pt::new(1.0, 2.0), Pt::new(1.0, 2.0));
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
            Transform::default().translate(5.0, 7.0) * Pt::new(1.0, -2.0),
            Pt::new(6.0, 5.0)
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
            Transform::default().scale(2.0, 3.0) * Pt::new(1.5, -2.0),
            Pt::new(3.0, -6.0)
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
            Transform::default().rotate(PI / 2.0) * Pt::new(15.0, 7.0),
            Pt::new(-7.0000005, 15.0)
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
            Transform::default().skew(0.0, PI / 4.0) * (5.0, 3.0),
            Pt::new(5.0, 8.0)
        );
        assert_eq!(
            Transform::default().skew(0.0, PI / 4.0) * Pt::new(15.0, 7.0),
            Pt::new(15.0, 22.0)
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
