#![feature(test)]
extern crate test;

use puid::Puid;

#[bench]
fn with_macro(b: &mut test::Bencher) {
    b.iter(|| {
        puid::puid!("foo");
    })
}

#[bench]
fn with_builder(b: &mut test::Bencher) {
    b.iter(|| {
        let _ = Puid::builder().prefix("foo").unwrap().build().unwrap();
    })
}
