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
    pts: [Pt<F>; 2],
}

/// Iterator for points in a bounding box
pub struct BBoxIter<F>
where
    F: Float,
{
    pts: [Pt<F>; 2],
    i: u8,
}

impl<F> BBoxIter<F>
where
    F: Float,
{
    fn new(pts: [Pt<F>; 2]) -> Self {
        Self { pts, i: 0 }
    }
}

impl<F> Iterator for BBoxIter<F>
where
    F: Float,
{
    type Item = Pt<F>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            self.i = 1;
            Some(self.pts[0])
        } else if self.i == 1 {
            self.i = 2;
            Some(self.pts[1])
        } else {
            None
        }
    }
}

impl<F> IntoIterator for BBox<F>
where
    F: Float,
{
    type Item = Pt<F>;
    type IntoIter = BBoxIter<F>;

    fn into_iter(self) -> Self::IntoIter {
        BBoxIter::new(self.pts)
    }
}

impl<F> Default for BBox<F>
where
    F: Float,
{
    fn default() -> Self {
        let minp = Pt::new(F::max_value(), F::max_value());
        let maxp = Pt::new(F::min_value(), F::min_value());
        let pts = [minp, maxp];
        Self { pts }
    }
}

impl<F> From<Pt<F>> for BBox<F>
where
    F: Float,
{
    fn from(pt: Pt<F>) -> Self {
        Self { pts: [pt, pt] }
    }
}

impl<F> From<&Pt<F>> for BBox<F>
where
    F: Float,
{
    fn from(pt: &Pt<F>) -> Self {
        Self { pts: [*pt, *pt] }
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
        let mut bbox = Self::default();
        bbox.extend(pts);
        bbox
    }

    /// Extend bounding box with a set of points
    pub fn extend<I, P>(&mut self, pts: I)
    where
        I: IntoIterator<Item = P>,
        P: Into<Pt<F>>,
    {
        pts.into_iter()
            .for_each(|p| self.include_pt(p));
    }

    fn include_pt<P>(&mut self, p: P)
    where
        P: Into<Pt<F>>,
    {
        let p = p.into();
        let minp = self.pts[0].with_min(p);
        let maxp = self.pts[1].with_max(p);
        self.pts = [minp, maxp];
    }

    /// Get the minimum X value
    pub fn x_min(self) -> F {
        self.pts[0].x()
    }

    /// Get the maximum X value
    pub fn x_max(self) -> F {
        self.pts[1].x()
    }

    /// Get the minimum Y value
    pub fn y_min(self) -> F {
        self.pts[0].y()
    }

    /// Get the maximum Y value
    pub fn y_max(self) -> F {
        self.pts[1].y()
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
        let a = BBox::from(&Pt::new(0.0, 0.0));
        assert_eq!(a.x_min(), 0.0);
        assert_eq!(a.x_max(), 0.0);
        assert_eq!(a.x_span(), 0.0);
        assert_eq!(a.y_min(), 0.0);
        assert_eq!(a.y_max(), 0.0);
        assert_eq!(a.y_span(), 0.0);
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

    #[test]
    fn extend() {
        let mut a = BBox::new([(0.0, 0.0), (1.0, 1.0)]);
        a.extend([(-1.0, -1.0)]);
        assert_eq!(a.x_min(), -1.0);
        assert_eq!(a.x_max(), 1.0);
        assert_eq!(a.x_span(), 2.0);
        assert_eq!(a.y_min(), -1.0);
        assert_eq!(a.y_max(), 1.0);
        assert_eq!(a.y_span(), 2.0);
        let mut a = BBox::default();
        a.extend([(0.0, 0.0)]);
        assert_eq!(a.x_min(), 0.0);
        assert_eq!(a.x_max(), 0.0);
        assert_eq!(a.x_span(), 0.0);
        assert_eq!(a.y_min(), 0.0);
        assert_eq!(a.y_max(), 0.0);
        assert_eq!(a.y_span(), 0.0);
    }
}
