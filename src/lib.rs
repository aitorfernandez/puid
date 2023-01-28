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
//! use puid::puid;
//!
//! fn main() {
//!     let id = puid!("foo"); // foo_l2ok01bl0yq2i2ElC7zWaCR8
//! }
//! ```
//!
//! ## Using custom random length
//!
//! ```rust
//! use puid::puid;
//!
//! fn main() {
//!     let id = puid!("bar", 24); // bar_l2ok1yvk1z4aOz1P7kecCTaqUGq1wgKfHGZC
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/puid")]
#![warn(missing_docs)]

#[allow(deprecated)]

/// ...
mod puid;

pub use crate::puid::*;

/// ...
pub mod error;
