//! Compatibility re-export for the concrete grep command builder.
//!
//! The active implementation lives under `utilscript::grep`; this module keeps
//! older `xccute::grep::GrepBuilder` imports from pointing at an empty surface.

pub use crate::utilscript::grep::GrepBuilder;
