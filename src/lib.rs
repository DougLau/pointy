// lib.rs      Pointy crate.
//
// Copyright (c) 2020  Douglas P Lau
//
//! Pointy is a minimal crate for 2D geometry providing points, lines, segments
//! and affine transformations.
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

mod line;
mod point;
mod transform;

pub use line::{Line32, Line64};
pub use point::{Pt32, Pt64};
pub use transform::{Transform32, Transform64};
