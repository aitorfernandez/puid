use crate::error::{PuidError, PuidResult};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{
    sync::atomic::{AtomicU8, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

// shared state that requires a stable memory location
static COUNTER: AtomicU8 = AtomicU8::new(0);

const BASE_36: u8 = 36;
const DEFAULT_ENTROPY: u8 = 12;
const PREFIX_MAX_LEN: usize = 8;
const PREFIX_MIN_LEN: usize = 1;

/// The exposed struct for generate Puids.
pub struct Puid;

impl Puid {
    /// Exposed method to use the builder.
    pub fn builder() -> PuidBuilder<'static> {
        PuidBuilder::new()
    }
}

/// A builder struct for constructing puids.
#[derive(Debug, Default)]
pub struct PuidBuilder<'a> {
    entropy: u8,
    prefix: &'a str,
}

impl<'a> PuidBuilder<'a> {
    /// Creates a new instance of `PuidBuilder` with default entropy.
    pub fn new() -> Self {
        Self {
            entropy: DEFAULT_ENTROPY,
            ..Self::default()
        }
    }

    /// Sets the prefix if it passes validation.
    pub fn prefix(mut self, prefix: &'a str) -> PuidResult<Self> {
        if validate(prefix) {
            self.prefix = prefix;
            Ok(self)
        } else {
            Err(PuidError::InvalidPrefix)
        }
    }

    /// Sets the entropy (length of random characters).
    pub fn entropy(mut self, entropy: u8) -> Self {
        self.entropy = entropy;
        self
    }

    /// Builds the final PUID string if prefix is valid.
    pub fn build(self) -> PuidResult<String> {
        if self.prefix.is_empty() {
            return Err(PuidError::InvalidPrefix);
        }

        // self.prefix.len() for the prefix,
        // 1 for the underscore _ separator
        // 16 for the time value in base-36 (which is a reasonable upper bound)
        // 3 for the counter value
        // 16 for the process ID in base-36
        // self.entropy for the random alphanumeric string
        let mut result =
            String::with_capacity(self.prefix.len() + 1 + 16 + 3 + 16 + self.entropy as usize);

        result.push_str(self.prefix);
        result.push('_');
        result.push_str(&to_base36(time()));
        result.push_str(&counter().to_string());
        result.push_str(&to_base36(std::process::id() as u128));
        result.push_str(&rnd_string(self.entropy));

        Ok(result)
    }
}

/// Generates a base-36 encoded string from a `u128` value.
fn to_base36(mut v: u128) -> String {
    // 16 characters cover most cases which is typical for base-36 encoding of a u128
    let mut result = String::with_capacity(16);
    while v > 0 {
        result.push(char::from_digit((v % BASE_36 as u128) as u32, BASE_36 as u32).unwrap());
        v /= BASE_36 as u128;
    }
    result.chars().rev().collect()
}

/// Generates a random alphanumeric string of the specified length.
fn rnd_string(elements: u8) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(elements as usize)
        .map(char::from)
        .collect()
}

/// Increments and fetches an atomic counter, resetting to 0 upon reaching u8::MAX.
fn counter() -> u8 {
    COUNTER
        .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |i| match i {
            i if i == u8::MAX => Some(0),
            _ => Some(i + 1),
        })
        .unwrap()
}

/// Retrieves the current system time in milliseconds since the UNIX epoch.
fn time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

/// Validates the prefix for length and alphanumeric characters.
fn validate(prefix: &str) -> bool {
    (PREFIX_MIN_LEN..=PREFIX_MAX_LEN).contains(&prefix.len())
        && prefix.chars().all(|c| c.is_ascii_alphanumeric())
}

#[doc(hidden)]
#[deprecated(since = "0.1.0", note = "Deprecated in favour of Puid::builder()")]
// Composes the different parts of the ID.
pub fn puid(pref: &str, elements: u8) -> String {
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
