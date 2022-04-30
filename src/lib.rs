use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{
    sync::atomic::{AtomicU8, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static BASE_36: u8 = 36;
static COUNTER: AtomicU8 = AtomicU8::new(0);

fn to_base36(mut v: u128) -> String {
    let mut chars = vec![];
    while v > 0 {
        chars.push(char::from_digit((v % BASE_36 as u128) as u32, BASE_36 as u32).unwrap());
        v /= BASE_36 as u128;
    }
    chars.into_iter().rev().collect()
}

fn rnd_string(elements: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(elements)
        .map(char::from)
        .collect()
}

fn counter() -> u8 {
    COUNTER
        .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |i| match i {
            i if i == u8::MAX - 1 => Some(0),
            _ => Some(i + 1),
        })
        .unwrap()
}

fn time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn validate(pref: &str) -> bool {
    match pref.chars().count() {
        1..=4 => pref.chars().all(|c| c.is_ascii_alphanumeric()),
        _ => false,
    }
}

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

#[macro_export]
macro_rules! puid {
    ($pref:expr) => {
        $crate::puid($pref, 12)
    };

    ($pref:expr, $elements:expr) => {
        $crate::puid($pref, $elements)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn validate_test() {
        let tests = HashMap::from([
            ("", ("f", true)),
            ("", ("fo", true)),
            ("", ("foo", true)),
            ("", ("quux", true)),
            ("", ("b4r", true)),
            ("", ("bäz", false)),
            ("", ("fo_o", false)),
            ("", ("", false)),
        ]);
        for (desc, t) in tests {
            assert_eq!(validate(t.0), t.1, "{}", desc);
        }
    }

    #[test]
    fn rnd_string_test() {
        assert_eq!(rnd_string(12).len(), 12);
    }
}
