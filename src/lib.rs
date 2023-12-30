//! For usage and reference, see the [mdbook-admonish book](https://tommilligan.github.io/mdbook-admonish/)
//!
//! Documentation is hosted externally, as docs.rs does not currently support plugins.

mod book_config;
mod config;
#[doc(hidden)]
pub mod custom;
mod markdown;
mod parse;
mod preprocessor;
mod render;
mod resolve;
mod types;

pub use crate::preprocessor::Admonish;
