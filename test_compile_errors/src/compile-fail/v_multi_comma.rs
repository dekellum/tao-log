//! Test of too many trailing commas

extern crate tao_log;
use tao_log::*;

fn multi_trailing_comma() {
    infov!(33,,);
}

fn main() {}
