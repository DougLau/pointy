// line.rs      2D Lines
//
// Copyright (c) 2020-2022  Douglas P Lau
//
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

    /// Get the distance from the line (as a segment) to a point
    pub fn segment_distance<P>(self, pt: P) -> F
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

    /// Get the point where two lines intersect
    pub fn intersection(self, rhs: Self) -> Option<Pt<F>> {
        let v0 = self.p1 - self.p0;
        let v1 = rhs.p1 - rhs.p0;
        let den = v0 * v1;
        if den != F::zero() {
            let v2 = self.p0 - rhs.p0;
            let num = v1 * v2;
            let u = num / den;
            let x = self.p0.x() + u * v0.x();
            let y = self.p0.y() + u * v0.y();
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
        let x1 = pt.x() + perp.x();
        let y1 = pt.y() + perp.y();
        let p1 = Pt::new(x1, y1);
        self.intersection(Self::new(pt, p1)).unwrap()
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
    fn segment() {
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
}
