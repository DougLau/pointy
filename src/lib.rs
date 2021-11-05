// lib.rs      Pointy crate.
//
// Copyright (c) 2020-2021  Douglas P Lau
//
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

mod bbox;
mod float;
mod line;
mod point;
mod transform;

pub use bbox::{BBox, BBoxIter};
pub use float::Float;
pub use line::Line;
pub use point::Pt;
pub use transform::Transform;
