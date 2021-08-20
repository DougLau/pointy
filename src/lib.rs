// lib.rs      Pointy crate.
//
// Copyright (c) 2020-2021  Douglas P Lau
//
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

mod bbox;
mod line;
mod point;
mod transform;

pub use bbox::{BBox32, BBox64};
pub use line::{Line32, Line64};
pub use point::{Pt32, Pt64};
pub use transform::{Transform32, Transform64};
