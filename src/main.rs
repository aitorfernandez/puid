use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::process;
use std::sync::atomic::{AtomicU8, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

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

fn rnd(elements: usize) -> String {
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
        1 | 2 | 3 => pref.chars().all(|c| c.is_ascii_alphanumeric()),
        _ => false,
    }
}

pub fn puid(pref: &str, elements: usize) -> String {
    assert!(
        validate(pref),
        "Prefix cannot be longer than 3 characters and non-alphanumeric characters"
    );

    // pref + 24 bytes = time 8 + counter 3 + process 4 + rand default 9
    format!(
        "{}_{}_{:0>3}_{}_{}",
        pref,
        to_base36(time()),
        counter(),
        to_base36(process::id() as u128),
        rnd(elements)
    )
}

#[macro_export]
macro_rules! puid {
    ($pref:expr) => {
        $crate::puid($pref, 9)
    };

    ($pref:expr, $elements:expr) => {
        $crate::puid($pref, $elements)
    };
}

fn main() {
    for _ in 0..10 {
        println!("> {}", puid!("pre"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn prefix_test() {
        let tests = HashMap::from([
            ("f", true),
            ("fo", true),
            ("foo", true),
            ("b4r", true),
            ("bäz", false),
            ("", false),
            ("quux", false),
        ]);
        for (pref, res) in tests {
            assert_eq!(validate(pref), res);
        }
    }
}
