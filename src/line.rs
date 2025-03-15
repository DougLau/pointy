// line.rs      2D Lines
//
// Copyright (c) 2020-2025  Douglas P Lau
//
use crate::float::Float;
use crate::point::Pt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Line of infinite length
///
/// ```rust
/// use pointy::Line;
///
/// let line = Line::new((10.0, 15.0), (0.0, 2.0));
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Line<F>
where
    F: Float,
{
    /// First point
    p0: Pt<F>,

    /// Second point
    p1: Pt<F>,
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

    /// Make canonical line.
    ///
    /// Two canonical lines can be compared for equality.
    pub fn canonical(self) -> Self {
        // lines for `x = 0`, `x = 1`, `y = 0`, `y = 1`
        let x0 = Line::new((F::zero(), F::zero()), (F::zero(), F::one()));
        let x1 = Line::new((F::one(), F::zero()), (F::one(), F::one()));
        let y0 = Line::new((F::zero(), F::zero()), (F::one(), F::zero()));
        let y1 = Line::new((F::zero(), F::one()), (F::one(), F::one()));
        match (self.intersection(x0), self.intersection(y0)) {
            (Some(p0), Some(p1)) => {
                // does the line pass through the origin?
                if p0 == p1 {
                    // yes, find point at `x = 1`
                    match self.intersection(x1) {
                        Some(p1) => Line::new(p0, p1),
                        None => unreachable!(),
                    }
                } else {
                    // no, these are the canonical points
                    Line::new(p0, p1)
                }
            }
            (Some(p0), None) => {
                // horizontal; find point at `x = 1`
                match self.intersection(x1) {
                    Some(p1) => Line::new(p0, p1),
                    None => unreachable!(),
                }
            }
            (None, Some(p0)) => {
                // vertical; find point at `y = 1`
                match self.intersection(y1) {
                    Some(p1) => Line::new(p0, p1),
                    None => unreachable!(),
                }
            }
            _ => unreachable!(),
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
    fn canonical() {
        let a = Line::new((0.0, 0.0), (1.0, 0.0));
        assert_eq!(a, a.canonical());
        let a1 = Line::new((0.0, 15.0), (37.0, 0.0));
        assert_eq!(a1, a1.canonical());
        let a2 = Line::new((23.0, 0.0), (0.0, -19.0));
        assert_eq!(a2.canonical(), Line::new((0.0, -19.0), (23.0, 0.0)));
        let h0 = Line::new((0.0, 0.0), (2.0, 0.0));
        assert_eq!(h0.canonical(), Line::new((0.0, 0.0), (1.0, 0.0)));
        let h1 = Line::new((0.0, 1.0), (5.0, 1.0));
        assert_eq!(h1.canonical(), Line::new((0.0, 1.0), (1.0, 1.0)));
        let v0 = Line::new((0.0, -1.0), (0.0, 1.0));
        assert_eq!(v0.canonical(), Line::new((0.0, 0.0), (0.0, 1.0)));
        let v1 = Line::new((1.0, -3.0), (1.0, 8.0));
        assert_eq!(v1.canonical(), Line::new((1.0, 0.0), (1.0, 1.0)));
    }
}
