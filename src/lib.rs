// lib.rs      Pointy crate.
//
// Copyright (c) 2020-2025  Douglas P Lau
//
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod bbox;
mod float;
mod line;
mod point;
mod segment;
mod transform;

pub use bbox::{BBox, BBoxIter, Bounded, Bounds};
pub use float::Float;
pub use line::Line;
pub use point::Pt;
pub use segment::Seg;
pub use transform::Transform;
