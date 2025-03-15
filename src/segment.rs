// segment.rs    2D Line Segments
//
// Copyright (c) 2020-2025  Douglas P Lau
//
use crate::bbox::{BBox, Bounded, Bounds};
use crate::float::Float;
use crate::line::Line;
use crate::point::Pt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Segment of a line between two endpoints
///
/// ```rust
/// use pointy::Seg;
///
/// let seg = Seg::new((10.0, 15.0), (0.0, 2.0));
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Seg<F>
where
    F: Float,
{
    /// First endpoint
    pub p0: Pt<F>,

    /// Second endpoint
    pub p1: Pt<F>,
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
            (
                Bounds::Left | Bounds::BelowLeft | Bounds::AboveLeft,
                Bounds::Left | Bounds::BelowLeft | Bounds::AboveLeft,
            ) => false,
            (
                Bounds::Right | Bounds::BelowRight | Bounds::AboveRight,
                Bounds::Right | Bounds::BelowRight | Bounds::AboveRight,
            ) => false,
            (
                Bounds::Below | Bounds::BelowLeft | Bounds::BelowRight,
                Bounds::Below | Bounds::BelowLeft | Bounds::BelowRight,
            ) => false,
            (
                Bounds::Above | Bounds::AboveLeft | Bounds::AboveRight,
                Bounds::Above | Bounds::AboveLeft | Bounds::AboveRight,
            ) => false,
            (Bounds::Left, _) | (_, Bounds::Left) => {
                self.intersects(bbox.x_min_edge())
            }
            (Bounds::Right, _) | (_, Bounds::Right) => {
                self.intersects(bbox.x_max_edge())
            }
            (Bounds::BelowLeft, _) | (_, Bounds::BelowLeft) => {
                self.intersects(bbox.x_min_edge())
                    || self.intersects(bbox.y_min_edge())
            }
            (Bounds::AboveLeft, _) | (_, Bounds::AboveLeft) => {
                self.intersects(bbox.x_min_edge())
                    || self.intersects(bbox.y_max_edge())
            }
            (Bounds::BelowRight, _) | (_, Bounds::BelowRight) => {
                self.intersects(bbox.x_max_edge())
                    || self.intersects(bbox.y_min_edge())
            }
            (Bounds::AboveRight, _) | (_, Bounds::AboveRight) => {
                self.intersects(bbox.x_max_edge())
                    || self.intersects(bbox.y_max_edge())
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

    /// Clip segment with a bounding box
    pub fn clip(mut self, bbox: BBox<F>) -> Option<Self> {
        if !self.bounded_by(bbox) {
            return None;
        }
        if let Some(p) = self.intersection(bbox.x_min_edge()) {
            let xmn = bbox.x_min();
            if self.p0.x < xmn {
                self.p0 = p;
            } else if self.p1.x < xmn {
                self.p1 = p;
            }
        }
        if let Some(p) = self.intersection(bbox.x_max_edge()) {
            let xmx = bbox.x_max();
            if self.p0.x > xmx {
                self.p0 = p;
            } else if self.p1.x > xmx {
                self.p1 = p;
            }
        }
        if let Some(p) = self.intersection(bbox.y_min_edge()) {
            let ymn = bbox.y_min();
            if self.p0.y < ymn {
                self.p0 = p;
            } else if self.p1.y < ymn {
                self.p1 = p;
            }
        }
        if let Some(p) = self.intersection(bbox.y_max_edge()) {
            let ymx = bbox.y_max();
            if self.p0.y > ymx {
                self.p0 = p;
            } else if self.p1.y > ymx {
                self.p1 = p;
            }
        }
        Some(self)
    }
}

// Private BBox helper functions
impl<F> BBox<F>
where
    F: Float,
{
    /// Get edge on X min side
    fn x_min_edge(self) -> Seg<F> {
        let xmn = self.x_min();
        Seg::new((xmn, self.y_min()), (xmn, self.y_max()))
    }

    /// Get edge on X max side
    fn x_max_edge(self) -> Seg<F> {
        let xmx = self.x_max();
        Seg::new((xmx, self.y_min()), (xmx, self.y_max()))
    }

    /// Get edge on Y min side
    fn y_min_edge(self) -> Seg<F> {
        let ymn = self.y_min();
        Seg::new((self.x_min(), ymn), (self.x_max(), ymn))
    }

    /// Get edge on Y max side
    fn y_max_edge(self) -> Seg<F> {
        let ymx = self.y_max();
        Seg::new((self.x_min(), ymx), (self.x_max(), ymx))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn distance() {
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
    fn intersection() {
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
    fn bounded() {
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
