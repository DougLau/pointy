// line.rs      2D Lines
//
// Copyright (c) 2020-2024  Douglas P Lau
//
use crate::bbox::{BBox, Bounded, Bounds};
use crate::float::Float;
use crate::point::Pt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A line
///
/// ```rust
/// use pointy::Line;
///
/// let line = Line::new((10.0, 15.0), (0.0, 2.0));
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Line<F>
where
    F: Float,
{
    /// First point
    pub p0: Pt<F>,

    /// Second point
    pub p1: Pt<F>,
}

/// A line segment
///
/// ```rust
/// use pointy::Seg;
///
/// let seg = Seg::new((10.0, 15.0), (0.0, 2.0));
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Seg<F>
where
    F: Float,
{
    /// First point
    pub p0: Pt<F>,

    /// Second point
    pub p1: Pt<F>,
}

impl<F> Line<F>
where
    F: Float,
{
    /// Create a new line
    pub fn new<P0, P1>(p0: P0, p1: P1) -> Self
    where
        P0: Into<Pt<F>>,
        P1: Into<Pt<F>>,
    {
        Self {
            p0: p0.into(),
            p1: p1.into(),
        }
    }

    /// Get the distance from the line to a point
    pub fn distance<P>(self, pt: P) -> F
    where
        P: Into<Pt<F>>,
    {
        let pt = pt.into();
        let v0 = self.p1 - self.p0;
        let v1 = pt - self.p0;
        (v0 * v1).abs() / v0.mag()
    }

    /// Get the point where two lines intersect
    pub fn intersection(self, rhs: Self) -> Option<Pt<F>> {
        let v0 = self.p1 - self.p0;
        let v1 = rhs.p1 - rhs.p0;
        let den = v0 * v1;
        if den != F::zero() {
            let v2 = self.p0 - rhs.p0;
            let num = v1 * v2;
            let u = num / den;
            let x = self.p0.x + u * v0.x;
            let y = self.p0.y + u * v0.y;
            Some(Pt::new(x, y))
        } else {
            None
        }
    }

    /// Project a point onto the line.
    ///
    /// Returns the point on the line nearest to the given point.
    pub fn project<P>(self, pt: P) -> Pt<F>
    where
        P: Into<Pt<F>>,
    {
        let pt = pt.into();
        let perp = (self.p1 - self.p0).right();
        let x1 = pt.x + perp.x;
        let y1 = pt.y + perp.y;
        let p1 = Pt::new(x1, y1);
        self.intersection(Self::new(pt, p1)).unwrap()
    }
}

impl<F> Bounded<F> for Seg<F>
where
    F: Float,
{
    fn bounded_by(self, bbox: BBox<F>) -> bool {
        let p0 = bbox.check(self.p0.x, self.p0.y);
        let p1 = bbox.check(self.p1.x, self.p1.y);
        match (p0, p1) {
            (Bounds::Within, _) | (_, Bounds::Within) => true,
            // both opposite horizontally
            (Bounds::Left, Bounds::Right) => true,
            (Bounds::Right, Bounds::Left) => true,
            // both opposite vertically
            (Bounds::Below, Bounds::Above) => true,
            (Bounds::Above, Bounds::Below) => true,
            // both on left side
            (
                Bounds::Left | Bounds::BelowLeft | Bounds::AboveLeft,
                Bounds::Left | Bounds::BelowLeft | Bounds::AboveLeft,
            ) => false,
            // both on right side
            (
                Bounds::Right | Bounds::BelowRight | Bounds::AboveRight,
                Bounds::Right | Bounds::BelowRight | Bounds::AboveRight,
            ) => false,
            // both below box
            (
                Bounds::Below | Bounds::BelowLeft | Bounds::BelowRight,
                Bounds::Below | Bounds::BelowLeft | Bounds::BelowRight,
            ) => false,
            // both above box
            (
                Bounds::Above | Bounds::AboveLeft | Bounds::AboveRight,
                Bounds::Above | Bounds::AboveLeft | Bounds::AboveRight,
            ) => false,
            // either point on left side
            (Bounds::Left | Bounds::BelowLeft | Bounds::AboveLeft, _)
            | (_, Bounds::Left | Bounds::BelowLeft | Bounds::AboveLeft) => {
                let xmn = bbox.x_min();
                // "left" edge of bounding box
                self.intersects(Seg::new(
                    (xmn, bbox.y_min()),
                    (xmn, bbox.y_max()),
                ))
            }
            // either point on right side
            (Bounds::Right | Bounds::BelowRight | Bounds::AboveRight, _)
            | (_, Bounds::Right | Bounds::BelowRight | Bounds::AboveRight) => {
                let xmx = bbox.x_max();
                // "right" edge of bounding box
                self.intersects(Seg::new(
                    (xmx, bbox.y_min()),
                    (xmx, bbox.y_max()),
                ))
            }
        }
    }
}

impl<F> Seg<F>
where
    F: Float,
{
    /// Create a new line segment
    pub fn new<P0, P1>(p0: P0, p1: P1) -> Self
    where
        P0: Into<Pt<F>>,
        P1: Into<Pt<F>>,
    {
        Self {
            p0: p0.into(),
            p1: p1.into(),
        }
    }

    /// Get the distance from the line segment to a point
    pub fn distance<P>(self, pt: P) -> F
    where
        P: Into<Pt<F>>,
    {
        let pt = pt.into();
        // If the dot product of `v0` and `v1` is greater than zero,
        // then the nearest point on the segment is `p1`
        let v0 = self.p1 - self.p0;
        let v1 = pt - self.p1;
        if v0.dot(v1) > F::zero() {
            return v1.mag();
        }
        // If the dot product of `v2` and `v3` is greater than zero,
        // then the nearest point on the segment is `p0`
        let v2 = self.p0 - self.p1;
        let v3 = pt - self.p0;
        if v2.dot(v3) > F::zero() {
            return v3.mag();
        }
        // Otherwise, the nearest point on the segment is between
        // `p0` and `p1`, so calculate the point-line distance
        (v0 * v3).abs() / v0.mag()
    }

    /// Get the point where two segments intersect
    pub fn intersection(self, rhs: Self) -> Option<Pt<F>> {
        let l0 = Line::new(self.p0, self.p1);
        let l1 = Line::new(rhs.p0, rhs.p1);
        l0.intersection(l1)
            .filter(|p| p.bounded_by(BBox::new([rhs.p0, rhs.p1])))
    }

    /// Check if segment intersects with another segment
    pub fn intersects(self, rhs: Self) -> bool {
        self.intersection(rhs).is_some()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn distance() {
        let a = Line::new((0.0, 0.0), (1.0, 0.0));
        assert_eq!(a.distance((0.0, 1.0)), 1.0);
        let b = Line::new((0.0, 0.0), (0.0, 1.0));
        assert_eq!(b.distance((2.0, 0.0)), 2.0);
    }

    #[test]
    fn intersection() {
        let a = Line::new((0.0, 0.0), (1.0, 0.0));
        assert_eq!(a.intersection(a), None);
        let c = Line::new((1.0, 1.0), (1.0, 0.0));
        assert_eq!(a.intersection(c), Some(Pt::new(1.0, 0.0)));
        let b = Line::new((0.0, 0.0), (0.0, 1.0));
        assert_eq!(b.intersection(b), None);
    }

    #[test]
    fn projection() {
        let d = Line::new((0.0, 0.0), (10.0, 0.0));
        assert_eq!(d.project((0.0, 5.0)), Pt::new(0.0, 0.0));
        assert_eq!(d.project((5.0, 5.0)), Pt::new(5.0, 0.0));
        assert_eq!(d.project((10.0, 5.0)), Pt::new(10.0, 0.0));
        assert_eq!(d.project((-5.0, 0.0)), Pt::new(-5.0, 0.0));
        assert_eq!(d.project((15.0, 0.0)), Pt::new(15.0, 0.0));
        assert_eq!(d.project((0.0, -5.0)), Pt::new(0.0, 0.0));
        assert_eq!(d.project((5.0, -5.0)), Pt::new(5.0, 0.0));
        assert_eq!(d.project((10.0, -5.0)), Pt::new(10.0, 0.0));
    }

    #[test]
    fn seg_dist() {
        let a = Seg::new((0.0, 0.0), (10.0, 0.0));
        assert_eq!(a.distance((0.0, 5.0)), 5.0);
        assert_eq!(a.distance((5.0, 5.0)), 5.0);
        assert_eq!(a.distance((10.0, 5.0)), 5.0);
        assert_eq!(a.distance((-5.0, 0.0)), 5.0);
        assert_eq!(a.distance((15.0, 0.0)), 5.0);
        assert_eq!(a.distance((0.0, -5.0)), 5.0);
        assert_eq!(a.distance((5.0, -5.0)), 5.0);
        assert_eq!(a.distance((10.0, -5.0)), 5.0);
    }

    #[test]
    fn seg_intersection() {
        let a = Seg::new((0.0, 0.0), (1.0, 0.0));
        assert_eq!(a.intersection(a), None);
        let b = Seg::new((1.0, 1.0), (1.0, 0.0));
        assert_eq!(a.intersection(b), Some(Pt::new(1.0, 0.0)));
        let c = Seg::new((0.5, 1.0), (0.5, 10.0));
        assert_eq!(a.intersection(c), None);
        let d = Seg::new((0.5, 1.0), (0.5, -1.0));
        assert_eq!(a.intersection(d), Some(Pt::new(0.5, 0.0)));
    }

    #[test]
    fn seg_bounded() {
        let b = BBox::new([(0.0, 0.0), (1.0, 1.0)]);
        assert!(Seg::new((0.0, 0.0), (1.0, 1.0)).bounded_by(b));
        assert!(Seg::new((1.0, 1.0), (2.0, 2.0)).bounded_by(b));
        assert!(Seg::new((0.0, 0.0), (-1.0, -1.0)).bounded_by(b));
        assert!(!Seg::new((2.0, 2.0), (3.0, 3.0)).bounded_by(b));
        assert!(!Seg::new((-1.0, -1.0), (-2.0, -2.0)).bounded_by(b));
        assert!(Seg::new((0.5, 0.5), (1.5, 0.5)).bounded_by(b));
        assert!(Seg::new((0.5, 0.5), (0.5, 1.5)).bounded_by(b));
        assert!(Seg::new((0.5, 1.5), (1.5, 0.5)).bounded_by(b));
        assert!(!Seg::new((0.5, 1.6), (1.6, 0.5)).bounded_by(b));
        assert!(Seg::new((-0.5, 0.5), (1.5, 0.5)).bounded_by(b));
        assert!(Seg::new((0.5, -0.5), (0.5, 1.5)).bounded_by(b));
    }
}
