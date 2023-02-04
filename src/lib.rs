//! A unique ID generator using a given prefix with `ch_`-style.
//!
//! The ID is a composition of:
//!
//! - Prefix.
//! - Underscore character.
//! - Timestamp turned into Base-36.
//! - A u8 type counter.
//! - The OS-assigned process identifier turned into Base-36.
//! - Sequence the random characters.
//!
//! # Examples
//!
//! ## Using the default random length
//!
//! ```rust
//! use puid::Puid;
//!
//! fn main() {
//!     let id = Puid::builder().prefix("foo").unwrap().build().unwrap(); // foo_l2ok01bl0yq2i2ElC7zWaCR8
//! }
//! ```
//!
//! ## Using custom random length
//!
//! ```rust
//! use puid::Puid;
//!
//! fn main() {
//!     let id = Puid::builder().prefix("bar").unwrap().entropy(24).build().unwrap(); // bar_l2ok1yvk1z4aOz1P7kecCTaqUGq1wgKfHGZC
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/puid")]
#![warn(missing_docs)]
#![allow(deprecated)]

/// The Puid module.
mod puid;

pub use crate::puid::{puid, Puid};

/// The types error.
pub mod error;
