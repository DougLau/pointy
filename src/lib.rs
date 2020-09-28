// lib.rs      Pointy crate.
//
// Copyright (c) 2020  Douglas P Lau
//
//! Pointy is a minimal 2D graphics library.
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

mod geom;

pub use geom::{Pt, PtB, Transform, TransformB};
