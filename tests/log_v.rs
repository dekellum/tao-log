//! Stateful tests of *v (inline expression value) log macros (debugv, etc.)

use tao_log::*;

use std::sync::{Arc, Mutex};
use log::{Level, LevelFilter, Log, Record, Metadata};

#[cfg(feature = "std")]
use log::set_boxed_logger;

#[cfg(not(feature = "std"))]
fn set_boxed_logger(logger: Box<Log>) -> Result<(), log::SetLoggerError> {
    log::set_logger(unsafe { &*Box::into_raw(logger) })
}

struct State {
    last_log: Mutex<Option<String>>,
}

fn last(state: &State) -> Option<String> {
    state.last_log.lock().unwrap().take()
}

struct Logger(Arc<State>);

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let msg = format!("{}", record.args());
        *self.0.last_log.lock().unwrap() = Some(msg);
        assert_eq!(record.file(), Some(file!()));
        assert!(record.line().is_some());
        let t = record.target().to_owned();
        assert!(t == "log_v" || t == "special", t);
        println!("{:5} {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

fn main() {
    let me = Arc::new(State { last_log: Mutex::new(None) });
    let a = me.clone();
    set_boxed_logger(Box::new(Logger(me))).unwrap();

    log::set_max_level(LevelFilter::Debug);

    info!("Start of test (test output follows)");

    // Simplest use
    let i = 32;
    assert_eq!(debugv!(i), 32);
    assert_eq!(last(&a), Some("i → 32".to_owned()));

    // More expression (note reformatting via `stringify!`)
    assert_eq!(debugv!(i+1), 33);
    assert_eq!(last(&a), Some("i + 1 → 33".to_owned()));

    // Use special target (note target assert in Logger::log)
    assert_eq!(errorv!(target: "special", i), 32);
    assert_eq!(last(&a), Some("i → 32".to_owned()));

    // Use custom format (note: Display format, hex output)
    assert_eq!(warnv!("index", "{:#x}", i), 32);
    assert_eq!(last(&a), Some("index i → 0x20".to_owned()));

    // Use both special target and custom format
    assert_eq!(errorv!(target: "special", "custom:", "{:05}", i), 32);
    assert_eq!(last(&a), Some("custom: i → 00032".to_owned()));

    // Explicit tuple for multiple expressions
    let j = 19;
    let (q, r) = debugv!((j/4, j%4));
    assert_eq!(q, 4);
    assert_eq!(r, 3);
    assert_eq!(last(&a), Some("(j / 4, j % 4) → (4, 3)".to_owned()));

    // Explicit tuple and custom prefix
    assert_eq!(debugv!("fifth", (j/5, j%5)), (3, 4));
    assert_eq!(last(&a), Some("fifth (j / 5, j % 5) → (3, 4)".to_owned()));

    // Syntactic edge case of single value tuple
    assert_eq!(debugv!((j,)), (19,));
    #[allow(unused_parens)] {
        // A trailing comma is required for compiler to interpret as
        // tuple. This is not a tuple!
        assert_eq!(debugv!((j)), 19);
    }

    // String and its default `Debug` formatting, by reference and move.
    let vt = "foo";
    assert_eq!(infov!(&vt), &"foo");
    assert_eq!(last(&a), Some("&vt → \"foo\"".to_owned()));
    assert_eq!(infov!(vt), "foo");
    assert_eq!(last(&a), Some("vt → \"foo\"".to_owned()));

    // Trace disabled, expression still returned, but no log
    let i = 2;
    assert_eq!(tracev!(i*4), 8);
    assert_eq!(last(&a), None);

    // v* macros expand and evaluate the expression exactly _once_.
    let mut o = Some(33);
    assert_eq!(debugv!(o.take()), Some(33));
    assert_eq!(last(&a), Some("o.take() → Some(33)".to_owned()));
    assert_eq!(debugv!(o.take()), None);
    assert_eq!(last(&a), Some("o.take() → None".to_owned()));

    // Use `logv!` and special target (asserted in Logger::log)
    let i = 3;
    assert_eq!(logv!(target: "special", Level::Info, i), 3);
    assert_eq!(last(&a), Some("i → 3".to_owned()));

    // logv, default target, tuple
    assert_eq!(logv!(Level::Warn, (i+1, i+2)).1, 5);
    assert_eq!(last(&a), Some("(i + 1, i + 2) → (4, 5)".to_owned()));

    // void function, statement position
    fn fvoid() {}
    debugv!(fvoid());
    assert_eq!(last(&a), Some("fvoid() → ()".to_owned()));

    // str function, with let binding.
    fn fstr() -> String { "returned".to_owned() }
    let s = debugv!(fstr());
    assert_eq!(s, "returned");
    assert_eq!(last(&a), Some("fstr() → \"returned\"".to_owned()));

    // wrapping assignments is possible, but not very interesting, given
    // rust doesn't emit the value, even with Copy types.
    let mut m = 1;
    assert_eq!(m, 1);
    infov!(m = 2);
    assert_eq!(m, 2);
    assert_eq!(last(&a), Some("m = 2 → ()".to_owned()));

    // trailing comma:
    let i = infov!(4,);
    infov!("trailing comma", i,);
    logv!(target: "special", Level::Info, i,);

    // ### compiler accepted mis-features ###
    //
    // These are pretty innocuous, just resulting in unexpected output, but
    // worth documentating. Proc-macros could produce sane compiler errors for
    // these in the future.

    // Extra space if empty prefix is given: No great way to check that at
    // compile time.
    debugv!("", 22);
    assert_eq!(last(&a), Some(" 22 → 22".to_owned()));

    // Non-string prefix literal is allowed
    warnv!(666, 1);
    assert_eq!(last(&a), Some("666 1 → 1".to_owned()));

    // 2 placeholders in aggregate, wrong params
    let i = 4;
    debugv!("prefix with {:?}", "without", i);
    assert_eq!(last(&a), Some("prefix with \"i\" 4 → without".to_owned()));

    // multiple trailing commas are allowed before rustc 1.32.0
    #[cfg(not(trailing_comma))] {
        let i = infov!(5,,);
        infov!("trailing comma", i,,);
        logv!(target: "special", Level::Info, i,,);
    }

    info!("End of test (passed)");
}
