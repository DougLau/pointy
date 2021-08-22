// bbox.rs      Bounding boxes
//
// Copyright (c) 2020-2021  Douglas P Lau
//
use crate::float::Float;
use crate::point::Pt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Axis-aligned bounding box
///
/// # Example
/// ```
/// use pointy::{BBox, Pt};
///
/// let p0 = Pt::new(-10.0, 0.0);
/// let p1 = Pt::new(10.0, 8.0);
/// let bbox = BBox::new([p0, p1]);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BBox<F>
where
    F: Float,
{
    minp: Pt<F>,
    maxp: Pt<F>,
}

impl<F> Default for BBox<F>
where
    F: Float,
{
    fn default() -> Self {
        let minp = Pt::new(F::max_value(), F::max_value());
        let maxp = Pt::new(F::min_value(), F::min_value());
        Self { minp, maxp }
    }
}

impl<F> From<Pt<F>> for BBox<F>
where
    F: Float,
{
    fn from(pt: Pt<F>) -> Self {
        Self { minp: pt, maxp: pt }
    }
}

impl<F, P> From<(P, P)> for BBox<F>
where
    F: Float,
    P: Into<Pt<F>>,
{
    fn from(pts: (P, P)) -> Self {
        Self::new([pts.0, pts.1])
    }
}

impl<F, P> From<[P; 2]> for BBox<F>
where
    F: Float,
    P: Into<Pt<F>> + Copy,
{
    fn from(pts: [P; 2]) -> Self {
        Self::new(pts)
    }
}

impl<F> BBox<F>
where
    F: Float,
{
    /// Create a new axis-aligned bounding box
    pub fn new<I, P>(pts: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<Pt<F>>,
    {
        pts.into_iter()
            .fold(Self::default(), |bb, p| bb.include_pt(p))
    }

    fn include_pt<P>(self, p: P) -> Self
    where
        P: Into<Pt<F>>,
    {
        let p = p.into();
        let minp = self.minp.with_min(p);
        let maxp = self.maxp.with_max(p);
        Self { minp, maxp }
    }

    /// Get the minimum X value
    pub fn x_min(self) -> F {
        self.minp.x()
    }

    /// Get the maximum X value
    pub fn x_max(self) -> F {
        self.maxp.x()
    }

    /// Get the minimum Y value
    pub fn y_min(self) -> F {
        self.minp.y()
    }

    /// Get the maximum Y value
    pub fn y_max(self) -> F {
        self.maxp.y()
    }

    /// Get the X span
    pub fn x_span(self) -> F {
        self.x_max() - self.x_min()
    }

    /// Get the Y span
    pub fn y_span(self) -> F {
        self.y_max() - self.y_min()
    }

    /// Check if it intersects with another bounding box
    pub fn intersects(self, rhs: Self) -> bool {
        self.x_min() <= rhs.x_max()
            && self.x_max() >= rhs.x_min()
            && self.y_min() <= rhs.y_max()
            && self.y_max() >= rhs.y_min()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bounds() {
        let b = BBox::new([(0.0, 10.0), (100.0, 200.0)]);
        assert_eq!(b.x_min(), 0.0);
        assert_eq!(b.x_max(), 100.0);
        assert_eq!(b.x_span(), 100.0);
        assert_eq!(b.y_min(), 10.0);
        assert_eq!(b.y_max(), 200.0);
        assert_eq!(b.y_span(), 190.0);
    }

    #[test]
    fn from_vec() {
        let pts = [
            Pt::new(5.2, 55.8),
            Pt::new(-58.8, 20.0),
            Pt::new(150.0, -240.0),
        ];
        let b = BBox::new(pts);
        assert_eq!(b.x_min(), -58.8);
        assert_eq!(b.x_max(), 150.0);
        assert_eq!(b.x_span(), 208.8);
        assert_eq!(b.y_min(), -240.0);
        assert_eq!(b.y_max(), 55.8);
        assert_eq!(b.y_span(), 295.8);
    }

    #[test]
    fn intersects() {
        let a = BBox::new([(0.0, 0.0), (1.0, 1.0)]);
        assert!(a.intersects(BBox::new([(0.0, 0.0), (5.0, 5.0)])));
        assert!(a.intersects(BBox::new([(-1.0, -1.0), (0.0, 0.0)])));
        assert!(a.intersects(BBox::new([(0.0, 0.5), (1.0, 1.0)])));
        assert!(a.intersects(BBox::new([(1.0, 1.0), (2.0, 2.0)])));
        assert!(!a.intersects(BBox::new([(1.1, 1.0), (2.0, 2.0)])));
        assert!(!a.intersects(BBox::new([(0.0, 10.0), (100.0, 200.0)])));
    }
}
