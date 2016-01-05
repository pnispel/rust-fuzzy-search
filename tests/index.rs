#![feature(test)]

extern crate fuzzy;
extern crate test;

use test::Bencher;

#[test]
fn it_works() {
    assert!(true);
}

#[bench]
fn bench(b: &mut Bencher) {
    b.iter(|| assert!(true));
}
