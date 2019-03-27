//! Tests of *v argument count

extern crate tao_log;
use tao_log::*;

fn zero_arguments_comma() {
    infov!(,); //~ ERROR expected identifier, found `,`
}
