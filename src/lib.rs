//! A unique ID generator using a given prefix `ch_`-style.
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
//! ## Usign the default random length
//!
//! ```rust
//! use puid::puid;
//!
//! fn main() {
//!     let id = puid!("foo"); // foo_l2ok01bl0yq2i2ElC7zWaCR8
//! }
//! ```
//!
//! ## Usign custom random length
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

use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{
    sync::atomic::{AtomicU8, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static BASE_36: u8 = 36;
static COUNTER: AtomicU8 = AtomicU8::new(0);

// Convert the given unsigned value to a string of Base-36.
fn to_base36(mut v: u128) -> String {
    let mut chars = vec![];
    while v > 0 {
        chars.push(char::from_digit((v % BASE_36 as u128) as u32, BASE_36 as u32).unwrap());
        v /= BASE_36 as u128;
    }
    chars.into_iter().rev().collect()
}

// Randomly generates a string of given elements length in the range of alphanumeric characters.
fn rnd_string(elements: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(elements)
        .map(char::from)
        .collect()
}

// Returns the COUNTER value or reset it if reach the maximum value
fn counter() -> u8 {
    COUNTER
        .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |i| match i {
            i if i == u8::MAX => Some(0),
            _ => Some(i + 1),
        })
        .unwrap()
}

// Returns the total number of whole miliseconds from Unix epoch
fn time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

// Validates that the given string be between 1 and 5 characters and be alphanumeric characters.
fn validate(pref: &str) -> bool {
    match pref.chars().count() {
        1..=4 => pref.chars().all(|c| c.is_ascii_alphanumeric()),
        _ => false,
    }
}

#[doc(hidden)]
// Composes the different parts of the ID.
pub fn puid(pref: &str, elements: usize) -> String {
    assert!(
        validate(pref),
        "Prefix cannot be longer than 4 characters and with non-alphanumeric characters"
    );

    [
        pref,
        "_",
        &to_base36(time()),
        &counter().to_string(),
        &to_base36(std::process::id() as u128).to_string(),
        &rnd_string(elements),
    ]
    .concat()
}

/// Abstract the ID generation for easy use...
///
/// The two flavours
///
/// # Default
///
/// ```rust,ignore
/// puid::puid!("foo");
/// ```
///
/// # With custom random length
///
/// ```rust,ignore
/// puid::puid!("bar", 24);
/// ```
#[macro_export]
macro_rules! puid {
    // Default puid with size of 12 random characters at the end.
    ($pref:expr) => {
        $crate::puid($pref, 12)
    };

    // puid with custom size of random characters at the end.
    ($pref:expr, $elements:expr) => {
        $crate::puid($pref, $elements)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;
    use std::thread;

    #[test]
    fn validate_test() {
        let tests = HashMap::from([
            ("Valid prefix for 1 character long", ("f", true)),
            ("Valid prefix for 2 character long", ("fo", true)),
            ("Valid prefix for 3 character long", ("foo", true)),
            ("Valid prefix for 4 character long", ("quux", true)),
            ("Valid prefix for alphanumeric characters", ("b4r", true)),
            (
                "Invalid prefix for non-alphanumeric characters",
                ("bäz", false),
            ),
            ("Invalid prefix with empty value", ("", false)),
        ]);
        for (desc, t) in tests {
            assert_eq!(validate(t.0), t.1, "{}", desc);
        }
    }

    #[test]
    fn rnd_string_test() {
        assert_eq!(rnd_string(12).len(), 12);
    }

    #[test]
    fn to_base36_test() {
        assert_eq!(to_base36(1651312057), "rb5cjd");
    }

    #[test]
    fn counter_test() {
        let a = counter(); // 0
        let b = counter();
        let _ = thread::spawn(move || {
            for _ in b + 1..=u8::MAX {
                let _ = counter();
            }
        });
        assert!(a + 1 == b);
        assert_eq!(counter(), 0);
    }
}
