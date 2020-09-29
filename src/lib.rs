// lib.rs      Pointy crate.
//
// Copyright (c) 2020  Douglas P Lau
//
//! Pointy is a minimal 2D geometry library.
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

mod point;
mod transform;

pub use point::{Pt, PtB};
pub use transform::{Transform, TransformB};
