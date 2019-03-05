//! Tests of *v trailing comma

extern crate tao_log;
use tao_log::*;

fn trailing_comma() {
    warnv!(33,); //~ ERROR expected identifier, found `,`
}
