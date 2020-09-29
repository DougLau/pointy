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
            pub const fn new(a: $ptty, b: $ptty) -> Self {
                Self(a, b)
            }

            /// Get the distance from the line to a point
            pub fn distance(self, pt: $ptty) -> $fty {
                let ba = self.1 - self.0;
                let ca = pt - self.0;
                (ba * ca).abs() / ba.mag()
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
                    let x = self.0.x() + u * (self.1.x() - self.0.x());
                    let y = self.0.y() + u * (self.1.y() - self.0.y());
                    Some($ptexp(x, y))
                } else {
                    None
                }
            }

            /// Project a point onto the line.
            ///
            /// Returns the point on the line nearest to the given point.
            pub fn project(self, pt: $ptty) -> $ptty {
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

    const A: Line32 = Line32::new(Pt32(0.0, 0.0), Pt32(1.0, 0.0));
    const B: Line64 = Line64::new(Pt64(0.0, 0.0), Pt64(0.0, 1.0));
    const C: Line32 = Line32::new(Pt32(1.0, 1.0), Pt32(1.0, 0.0));
	const D: Line64 = Line64::new(Pt64(0.0, 0.0), Pt64(10.0, 0.0));

    #[test]
    fn distance() {
        assert_eq!(A.distance(Pt32(0.0, 1.0)), 1.0);
        assert_eq!(B.distance(Pt64(2.0, 0.0)), 2.0);
    }

    #[test]
    fn intersection() {
        assert_eq!(A.intersection(A), None);
        assert_eq!(A.intersection(C), Some(Pt32(1.0, 0.0)));
        assert_eq!(B.intersection(B), None);
    }

    #[test]
	fn projection() {
		assert_eq!(D.project(Pt64(0.0, 5.0)), Pt64(0.0, 0.0));
		assert_eq!(D.project(Pt64(5.0, 5.0)), Pt64(5.0, 0.0));
		assert_eq!(D.project(Pt64(10.0, 5.0)), Pt64(10.0, 0.0));
		assert_eq!(D.project(Pt64(-5.0, 0.0)), Pt64(-5.0, 0.0));
		assert_eq!(D.project(Pt64(15.0, 0.0)), Pt64(15.0, 0.0));
		assert_eq!(D.project(Pt64(0.0, -5.0)), Pt64(0.0, 0.0));
		assert_eq!(D.project(Pt64(5.0, -5.0)), Pt64(5.0, 0.0));
		assert_eq!(D.project(Pt64(10.0, -5.0)), Pt64(10.0, 0.0));
	}
}
