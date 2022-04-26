#![feature(test)]
extern crate test;

use test::Bencher;

#[bench]
fn default(b: &mut Bencher) {
    b.iter(|| {
        puid::puid!("foo");
    })
}
