#![forbid(missing_docs)]
#![forbid(missing_crate_level_docs)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_doc_code_examples)]
#![allow(clippy::derivable_impls)]
#![doc = include_str!("../readme.md")]

// mod writer;
//
pub use self::view::{iter::CodeViewIter, CodeSpan, CodeView};

// mod errors;
// mod palette;
mod view;
