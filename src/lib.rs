#![doc = include_str!("../README.md")]

#[cfg(feature = "generative")]
pub use yatl_core::generative;

pub use yatl_core::parser;

// Macro guide: https://towardsdatascience.com/nine-rules-for-creating-procedural-macros-in-rust-595aa476a7ff