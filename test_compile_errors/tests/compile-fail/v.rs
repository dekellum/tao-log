//! Tests of *v (inline expression value) log macro compile failures

extern crate tao_log;
use tao_log::*;

fn heap_string_as_prefix() {
    let ctx: String = "contextual prefix".to_owned();
    warnv!(&ctx, 1); //~ ERROR expected a literal
}

fn extra_placeholder_in_prefix() {
    let i = 4;
    warnv!("bad output verbatim {:?}", i); //~ ERROR 3 positional arguments in format string, but there are 2 arguments
}

fn trailing_comma() {
    warnv!(33,); //~ ERROR expected identifier, found `,`
}
