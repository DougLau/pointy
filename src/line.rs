// line.rs      2D Lines
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::point::{Pt32, Pt64};

/// 2-dimensional line with `f32` values.
///
/// ```rust
/// use pointy::{Line32, Pt32};
///
/// let line = Line32::new(Pt32(10.0, 15.0), Pt32(0.0, 2.0));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Line32(Pt32, Pt32);

/// 2-dimensional line with `f64` values.
///
/// ```rust
/// use pointy::{Line64, Pt64};
///
/// let line = Line64::new(Pt64(10.0, 15.0), Pt64(0.0, 2.0));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Line64(Pt64, Pt64);

macro_rules! define_line {
    ($lnty:ty, $ptty:ty, $ptexp:expr, $fty:ty) => {
        impl $lnty {
            /// Create a new line
            pub fn new<P0, P1>(a: P0, b: P1) -> Self
            where
                P0: Into<$ptty>,
                P1: Into<$ptty>,
            {
                Self(a.into(), b.into())
            }

            /// Get the distance from the line to a point
            pub fn distance<P: Into<$ptty>>(self, pt: P) -> $fty {
                let pt = pt.into();
                let v0 = self.1 - self.0;
                let v1 = pt - self.0;
                (v0 * v1).abs() / v0.mag()
            }

            /// Get the distance from the line (as a segment) to a point
            pub fn segment_distance<P: Into<$ptty>>(self, pt: P) -> $fty {
                let pt = pt.into();
                // If the dot product of `v0` and `v1` is greater than zero,
                // then the nearest point on the segment is `self.1`
                let v0 = self.1 - self.0;
                let v1 = pt - self.1;
                if v0.dot(v1) > 0.0 {
                    return v1.mag();
                }
                // If the dot product of `v2` and `v3` is greater than zero,
                // then the nearest point on the segment is `self.0`
                let v2 = self.0 - self.1;
                let v3 = pt - self.0;
                if v2.dot(v3) > 0.0 {
                    return v3.mag();
                }
                // Otherwise, the nearest point on the segment is between
                // `self.0` and `self.1`, so calculate the point-line distance
                (v0 * v3).abs() / v0.mag()
            }

            /// Get the point where two lines intersect
            pub fn intersection(self, rhs: Self) -> Option<$ptty> {
                let v0 = self.1 - self.0;
                let v1 = rhs.1 - rhs.0;
                let den = v0 * v1;
                if den != 0.0 {
                    let v2 = self.0 - rhs.0;
                    let num = v1 * v2;
                    let u = num / den;
                    let x = self.0.x() + u * v0.x();
                    let y = self.0.y() + u * v0.y();
                    Some($ptexp(x, y))
                } else {
                    None
                }
            }

            /// Project a point onto the line.
            ///
            /// Returns the point on the line nearest to the given point.
            pub fn project<P: Into<$ptty>>(self, pt: P) -> $ptty {
                let pt = pt.into();
                let perp = (self.1 - self.0).right();
                let x1 = pt.x() + perp.x();
                let y1 = pt.y() + perp.y();
                self.intersection(Self::new(pt, $ptexp(x1, y1))).unwrap()
            }
        }
    };
}

define_line!(Line32, Pt32, Pt32, f32);
define_line!(Line64, Pt64, Pt64, f64);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn distance() {
        let a = Line32::new((0.0, 0.0), (1.0, 0.0));
        assert_eq!(a.distance((0.0, 1.0)), 1.0);
        let b = Line64::new((0.0, 0.0), (0.0, 1.0));
        assert_eq!(b.distance((2.0, 0.0)), 2.0);
    }

    #[test]
    fn intersection() {
        let a = Line32::new((0.0, 0.0), (1.0, 0.0));
        assert_eq!(a.intersection(a), None);
        let c = Line32::new((1.0, 1.0), (1.0, 0.0));
        assert_eq!(a.intersection(c), Some(Pt32(1.0, 0.0)));
        let b = Line64::new((0.0, 0.0), (0.0, 1.0));
        assert_eq!(b.intersection(b), None);
    }

    #[test]
    fn projection() {
        let d = Line64::new((0.0, 0.0), (10.0, 0.0));
        assert_eq!(d.project((0.0, 5.0)), Pt64(0.0, 0.0));
        assert_eq!(d.project((5.0, 5.0)), Pt64(5.0, 0.0));
        assert_eq!(d.project((10.0, 5.0)), Pt64(10.0, 0.0));
        assert_eq!(d.project((-5.0, 0.0)), Pt64(-5.0, 0.0));
        assert_eq!(d.project((15.0, 0.0)), Pt64(15.0, 0.0));
        assert_eq!(d.project((0.0, -5.0)), Pt64(0.0, 0.0));
        assert_eq!(d.project((5.0, -5.0)), Pt64(5.0, 0.0));
        assert_eq!(d.project((10.0, -5.0)), Pt64(10.0, 0.0));
    }

    #[test]
    fn segment() {
        let a = Line64::new((0.0, 0.0), (10.0, 0.0));
        assert_eq!(a.segment_distance((0.0, 5.0)), 5.0);
        assert_eq!(a.segment_distance((5.0, 5.0)), 5.0);
        assert_eq!(a.segment_distance((10.0, 5.0)), 5.0);
        assert_eq!(a.segment_distance((-5.0, 0.0)), 5.0);
        assert_eq!(a.segment_distance((15.0, 0.0)), 5.0);
        assert_eq!(a.segment_distance((0.0, -5.0)), 5.0);
        assert_eq!(a.segment_distance((5.0, -5.0)), 5.0);
        assert_eq!(a.segment_distance((10.0, -5.0)), 5.0);
    }
}
