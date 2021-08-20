// bbox.rs      Bounding boxes
//
// Copyright (c) 2020-2021  Douglas P Lau
//
use crate::point::{Pt32, Pt64};

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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BBox32(Pt32, Pt32);

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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BBox64(Pt64, Pt64);

macro_rules! define_bbox {
    ($bxty:ty, $fty:ty, $ptty:ty) => {
        impl From<($ptty, $ptty)> for $bxty {
            fn from(pts: ($ptty, $ptty)) -> Self {
                let pt0 = pts.0.with_min(pts.1);
                let pt1 = pts.0.with_max(pts.1);
                Self(pt0, pt1)
            }
        }

        impl $bxty {
            /// Get the minimum X value
            pub fn x_min(self) -> $fty {
                self.0.x()
            }

            /// Get the maximum X value
            pub fn x_max(self) -> $fty {
                self.1.x()
            }

            /// Get the minimum Y value
            pub fn y_min(self) -> $fty {
                self.0.y()
            }

            /// Get the maximum Y value
            pub fn y_max(self) -> $fty {
                self.1.y()
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
    fn bbox_bounds() {
        let b =
            BBox64::from((Pt64::from((0.0, 10.0)), Pt64::from((100.0, 200.0))));
        assert_eq!(b.x_min(), 0.0);
        assert_eq!(b.x_max(), 100.0);
        assert_eq!(b.x_span(), 100.0);
        assert_eq!(b.y_min(), 10.0);
        assert_eq!(b.y_max(), 200.0);
        assert_eq!(b.y_span(), 190.0);
    }
}
