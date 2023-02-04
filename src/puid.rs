use crate::error::{PuidError, PuidResult};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{
    sync::atomic::{AtomicU8, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static BASE_36: u8 = 36;
static COUNTER: AtomicU8 = AtomicU8::new(0);

/// A builder struct for constructing puids.
#[derive(Debug, Default)]
pub struct PuidBuilder<'a> {
    entropy: usize,
    prefix: &'a str,
}

impl<'a> PuidBuilder<'a> {
    /// Start building a puid with default entropy.
    pub fn new() -> Self {
        Self {
            entropy: 12,
            ..Self::default()
        }
    }

    /// Add the puid prefix.
    pub fn prefix(self, prefix: &'a str) -> PuidResult<Self> {
        if validate(prefix) {
            Ok(Self { prefix, ..self })
        } else {
            Err(PuidError::InvalidPrefix)
        }
    }

    /// Add the custom entropy.
    pub fn entropy(self, entropy: usize) -> Self {
        Self { entropy, ..self }
    }

    /// Generate a string puid.
    pub fn build(self) -> PuidResult<String> {
        if self.prefix.is_empty() {
            Err(PuidError::InvalidPrefix)
        } else {
            Ok([
                self.prefix,
                "_",
                &to_base36(time()),
                &counter().to_string(),
                &to_base36(std::process::id() as u128),
                &rnd_string(self.entropy),
            ]
            .concat())
        }
    }
}

/// The exposed struct for generate Puids.
pub struct Puid;

impl Puid {
    /// Exposed method to use the builder.
    pub fn builder() -> PuidBuilder<'static> {
        PuidBuilder::new()
    }
}

// Convert the given unsigned value to a string of Base-36.
fn to_base36(mut v: u128) -> String {
    let mut chars = vec![];
    while v > 0 {
        chars.push(char::from_digit((v % BASE_36 as u128) as u32, BASE_36 as u32).unwrap());
        v /= BASE_36 as u128;
    }
    chars.into_iter().rev().collect()
}

// Randomly generates a string of given length in the range of alphanumeric characters.
fn rnd_string(elements: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(elements)
        .map(char::from)
        .collect()
}

// Returns the COUNTER value or reset it if it reaches the maximum value.
fn counter() -> u8 {
    COUNTER
        .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |i| match i {
            i if i == u8::MAX => Some(0),
            _ => Some(i + 1),
        })
        .unwrap()
}

// Returns the total number of whole milliseconds from Unix epoch.
fn time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

// Validates the given string, between 1 and 8 characters and only alphanumeric characters.
fn validate(pref: &str) -> bool {
    match pref.chars().count() {
        1..=8 => pref.chars().all(|c| c.is_ascii_alphanumeric()),
        _ => false,
    }
}

#[doc(hidden)]
#[deprecated(since = "0.1.0", note = "Deprecated in favour of Puid::builder()")]
// Composes the different parts of the ID.
pub fn puid(pref: &str, elements: usize) -> String {
    assert!(
        validate(pref),
        "Prefix cannot be longer than 4 characters and with non-alphanumeric characters."
    );

    [
        pref,
        "_",
        &to_base36(time()),
        &counter().to_string(),
        &to_base36(std::process::id() as u128),
        &rnd_string(elements),
    ]
    .concat()
}

/// Abstract the ID generation for easy usage.
///
/// With default size of 12 random characters at the end.
///
/// ```rust
/// puid::puid!("foo");
/// ```
///
/// With custom size of 24 random characters at the end.
///
/// ```rust
/// puid::puid!("bar", 24);
/// ```
#[macro_export]
#[deprecated(since = "0.1.0", note = "Deprecated in favour of Puid::builder()")]
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
    fn to_base36_test() {
        assert_eq!(to_base36(1651312057), "rb5cjd");
    }

    #[test]
    fn rnd_string_test() {
        assert_eq!(rnd_string(12).len(), 12);
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
        assert_eq!(counter(), 2);
    }

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
            assert_eq!(validate(t.0), t.1, "{desc}");
        }
    }

    #[test]
    fn puid_builder_test() {
        let id = Puid::builder().prefix("foo").unwrap().build();
        assert!(id.is_ok());
    }
}
