//! Tests of *v argument count

extern crate tao_log;
use tao_log::*;

fn zero_arguments() {
    infov!(); //~ ERROR unexpected end of macro invocation
}

fn four_arguments() {
    infov!("prefix", "{}", true, 4); //~ ERROR expected identifier, found `,`
}
