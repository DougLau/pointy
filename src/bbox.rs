// bbox.rs      Bounding boxes
//
// Copyright (c) 2020-2021  Douglas P Lau
//
use crate::point::{Pt32, Pt64};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Axis-aligned bounding box with [Pt32] points.
///
/// [Pt32]: struct.Pt32.html
///
/// # Example
/// ```
/// use pointy::{BBox32, Pt32};
///
/// let p0 = Pt32(-10.0, 0.0);
/// let p1 = Pt32(10.0, 8.0);
/// let bbox = BBox32::from((p0, p1));
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BBox32 {
    minp: Pt32,
    maxp: Pt32,
}

/// Axis-aligned bounding box with [Pt64] points.
///
/// [Pt64]: struct.Pt64.html
///
/// # Example
/// ```
/// use pointy::{BBox64, Pt64};
///
/// let p0 = Pt64(-10.0, 0.0);
/// let p1 = Pt64(10.0, 8.0);
/// let bbox = BBox64::from((p0, p1));
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BBox64 {
    minp: Pt64,
    maxp: Pt64,
}

macro_rules! define_bbox {
    ($bxty:ty, $fty:ty, $ptty:ty) => {
        impl Default for $bxty {
            fn default() -> Self {
                let minp = <$ptty>::from((<$fty>::MAX, <$fty>::MAX));
                let maxp = <$ptty>::from((<$fty>::MIN, <$fty>::MIN));
                Self { minp, maxp }
            }
        }

        impl From<$ptty> for $bxty {
            fn from(pt: $ptty) -> Self {
                Self { minp: pt, maxp: pt }
            }
        }

        impl From<($ptty, $ptty)> for $bxty {
            fn from(pts: ($ptty, $ptty)) -> Self {
                let minp = pts.0.with_min(pts.1);
                let maxp = pts.0.with_max(pts.1);
                Self { minp, maxp }
            }
        }

        impl $bxty {
            /// Create a new axis-aligned bounding box
            pub fn new<'a, I>(pts: I) -> Self
            where
                I: IntoIterator<Item = &'a $ptty>,
            {
                pts.into_iter()
                    .fold(Self::default(), |bb, p| bb.include_pt(*p))
            }

            fn include_pt(self, p: $ptty) -> Self {
                let minp = self.minp.with_min(p);
                let maxp = self.maxp.with_max(p);
                Self { minp, maxp }
            }

            /// Get the minimum X value
            pub fn x_min(self) -> $fty {
                self.minp.x()
            }

            /// Get the maximum X value
            pub fn x_max(self) -> $fty {
                self.maxp.x()
            }

            /// Get the minimum Y value
            pub fn y_min(self) -> $fty {
                self.minp.y()
            }

            /// Get the maximum Y value
            pub fn y_max(self) -> $fty {
                self.maxp.y()
            }

            /// Get the X span
            pub fn x_span(self) -> $fty {
                self.x_max() - self.x_min()
            }

            /// Get the Y span
            pub fn y_span(self) -> $fty {
                self.y_max() - self.y_min()
            }
        }
    };
}

define_bbox!(BBox32, f32, Pt32);
define_bbox!(BBox64, f64, Pt64);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bounds() {
        let b =
            BBox64::from((Pt64::from((0.0, 10.0)), Pt64::from((100.0, 200.0))));
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
            Pt64::from((5.2, 55.8)),
            Pt64::from((-58.8, 20.0)),
            Pt64::from((150.0, -240.0)),
        ];
        let b = BBox64::new(&pts);
        assert_eq!(b.x_min(), -58.8);
        assert_eq!(b.x_max(), 150.0);
        assert_eq!(b.x_span(), 208.8);
        assert_eq!(b.y_min(), -240.0);
        assert_eq!(b.y_max(), 55.8);
        assert_eq!(b.y_span(), 295.8);
    }
}
