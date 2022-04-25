use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::process;
use std::sync::atomic::{AtomicU16, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static COUNTER: AtomicU16 = AtomicU16::new(0);

fn rnd(elements: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(elements)
        .map(char::from)
        .collect()
}

fn counter() -> u16 {
    COUNTER
        .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |i| match i {
            i if i == u16::MAX - 1 => Some(0),
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

fn puid(prefix: &str, elements: usize) -> String {
    format!(
        "{}_{}_{:0>5}_{}_{}",
        prefix,
        time(),
        counter(),
        process::id(),
        rnd(elements)
    )
}

#[macro_export]
macro_rules! puid {
    ($prefix:expr) => {
        $crate::puid($prefix, 12)
    };

    ($prefix:expr, $elements:expr) => {
        $crate::puid($prefix, $elements)
    };
}

fn main() {
    for _ in 0..10 {
        println!("> {}", puid!("prefix"));
    }
}
