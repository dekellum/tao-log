#![feature(test)]
extern crate test; // Still required, see rust-lang/rust#55133
use test::Bencher;

use tao_log::*;
use tao_log::log::{Log, Record, Metadata};

use std::sync::Once;

struct BenchLogger;

impl Log for BenchLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let msg = format!("{}", record.args());
        // println!("{}", msg);
        assert!(msg.len() > 0);
    }

    fn flush(&self) {}
}

fn bench_logger() {
    static LOG_INIT: Once = Once::new();
    static LOGGER: BenchLogger = BenchLogger;
    LOG_INIT.call_once(|| {
        log::set_logger(&LOGGER).unwrap();
        log::set_max_level(log::LevelFilter::Info);
    });
}

#[bench]
fn b00_int_on(b: &mut Bencher)
{
    bench_logger();
    let ivar = 10;
    b.iter(|| {
        let j = ivar + 1;
        info!("ivar + 1 → {:?}", j);
        assert_eq!(11, j);
    })
}

#[bench]
fn b00_int_on_v(b: &mut Bencher)
{
    bench_logger();
    let ivar = 10;
    b.iter(|| {
        let j = infov!(ivar + 1);
        assert_eq!(11, j);
    })
}

#[bench]
fn b10_str_on(b: &mut Bencher)
{
    bench_logger();
    let svar = "string";
    b.iter(|| {
        info!("prefix svar → {}", svar);
        assert_eq!(svar.len(), 6);
    })
}

#[bench]
fn b10_str_on_v(b: &mut Bencher)
{
    bench_logger();
    let svar = "string";
    b.iter(|| {
        assert_eq!(infov!("prefix", "{}", svar).len(), 6);
    })
}

#[bench]
fn b11_string_on(b: &mut Bencher)
{
    bench_logger();
    let svar = "string";
    b.iter(|| {
        let s = svar.to_owned();
        info!("prefix svar.to_owned() → {:?}", s);
        assert_eq!(s.len(), 6);
    })
}

#[bench]
fn b11_string_on_v(b: &mut Bencher)
{
    bench_logger();
    let svar = "string";
    b.iter(|| {
        assert_eq!(infov!(svar.to_owned()).len(), 6);
    })
}

#[bench]
fn b20_mix_off_10x(b: &mut Bencher)
{
    bench_logger();
    let ivar = 10;
    let svar = "string";
    b.iter(|| {
        for _ in 1..10 {
            let j = ivar + 1;
            trace!("ivar + 1 → {:?}", j);
            assert_eq!(11, j);
            trace!("svar → {:?}", svar);
            trace!("prefix svar → {}", svar);
        }
    })
}

#[bench]
fn b20_mix_off_10x_v(b: &mut Bencher)
{
    bench_logger();
    let ivar = 10;
    let svar = "string";
    b.iter(|| {
        for _ in 1..10 {
            assert_eq!(11, tracev!(ivar + 1));
            tracev!(svar);
            tracev!("prefix", "{}", svar);
        }
    })
}
