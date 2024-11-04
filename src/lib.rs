//! A unique ID generator with customizable prefix, following a `prefix_` style.
//!
//! ## ID Structure
//!
//! The generated ID is composed of:
//!
//! - **Prefix**: A user-defined alphanumeric prefix.
//! - **Separator**: An underscore (`_`) character.
//! - **Timestamp**: The current timestamp, encoded in Base-36.
//! - **Counter**: An atomic `u8` counter to ensure unique IDs in rapid succession.
//! - **Process ID**: The OS-assigned process identifier, encoded in Base-36.
//! - **Random Sequence**: A customizable sequence of random alphanumeric characters, providing additional entropy.
//!
//! # Examples
//!
//! ## Using the default random sequence length
//!
//! ```rust
//! use puid::Puid;
//!
//! fn main() {
//!     let id = Puid::builder()
//!         .prefix("foo")
//!         .unwrap()
//!         .build()
//!         .unwrap();
//!     // Example output: "foo_l2ok01bl0yq2i2ElC7zWaCR8"
//! }
//! ```
//!
//! ## Using a custom random sequence length
//!
//! ```rust
//! use puid::Puid;
//!
//! fn main() {
//!     let id = Puid::builder()
//!         .prefix("bar")
//!         .unwrap()
//!         .entropy(24)
//!         .build()
//!         .unwrap();
//!     // Example output: "bar_l2ok1yvk1z4aOz1P7kecCTaqUGq1wgKfHGZC"
//! }
//! ```
//!
//! # Error Handling
//!
//! `Puid::builder` returns a `Result` indicating potential errors in configuration, such as an invalid prefix. The prefix must be alphanumeric and 1-8 characters long.

#![doc(html_root_url = "https://docs.rs/puid")]
#![warn(missing_docs)]
#![allow(deprecated)]

/// The Puid module.
mod puid;

pub use crate::puid::{puid, Puid};

/// The type error.
pub mod error;
