
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
fn run_log_on(b: &mut Bencher)
{
    bench_logger();
    let ivar = 10;
    let svar = "string";
    b.iter(|| {
        info!("ivar → {:?}", ivar);
        info!("svar → {:?}", svar);
        info!("prefix svar → {}", svar);
    })
}

#[bench]
fn run_logv_on(b: &mut Bencher)
{
    bench_logger();
    let ivar = 10;
    let svar = "string";
    b.iter(|| {
        assert_eq!(10, infov!(ivar));
        infov!(svar);
        infov!("prefix", "{}", svar);
    })
}

#[bench]
fn run_log_off(b: &mut Bencher)
{
    bench_logger();
    let ivar = 10;
    let svar = "string";
    b.iter(|| {
        trace!("ivar → {:?}", ivar);
        trace!("svar → {:?}", svar);
        trace!("prefix svar → {}", svar);
    })
}

#[bench]
fn run_logv_off(b: &mut Bencher)
{
    bench_logger();
    let ivar = 10;
    let svar = "string";
    b.iter(|| {
        assert_eq!(10, tracev!(ivar));
        tracev!(svar);
        tracev!("prefix", "{}", svar);
    })
}
