#![feature(test)]

extern crate test;

use std::fs;
use test::Bencher;

#[bench]
fn bench_taplo_parse_validate(b: &mut Bencher) {
    let src = fs::read_to_string("../test-data/example.toml").unwrap();
    b.iter(|| taplo::parser::parse(&src).into_dom());
}

#[bench]
fn bench_taplo_parse(b: &mut Bencher) {
    let src = fs::read_to_string("../test-data/example.toml").unwrap();
    b.iter(|| taplo::parser::parse(&src));
}

#[bench]
fn bench_toml_rs(b: &mut Bencher) {
    let src = fs::read_to_string("../test-data/example.toml").unwrap();
    b.iter(|| toml::from_str::<toml::Value>(&src).unwrap());
}
