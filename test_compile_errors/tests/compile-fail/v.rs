//! Tests of -v macro prefix errors

extern crate tao_log;
use tao_log::*;

fn heap_string_as_prefix() {
    let ctx: String = "contextual prefix".to_owned();
    warnv!(&ctx, 1);
}

fn extra_placeholder_in_prefix() {
    let i = 4;
    warnv!("bad prefix with {:?}", i);
}

fn missing_placeholder_in_value_format() {
    let i = 4;
    warnv!("prefix", "no place", i);
}

fn main() {}
