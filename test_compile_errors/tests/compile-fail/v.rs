//! Tests of -v macro prefix errors

extern crate tao_log;
use tao_log::*;

fn heap_string_as_prefix() {
    let ctx: String = "contextual prefix".to_owned();
    warnv!(&ctx, 1); //~ ERROR expected a literal
}

fn extra_placeholder_in_prefix() {
    let i = 4;
    warnv!("bad prefix with {:?}", i); //~ ERROR 3 positional arguments in format string, but there are 2 arguments
}

fn missing_placeholder_in_value_format() {
    let i = 4;
    warnv!("prefix", "no place", i); //~ ERROR argument never used
}
