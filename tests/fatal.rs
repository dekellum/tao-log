//! Tests of fatal macro

use std::cell::RefCell;
use std::sync::{Once, Arc};

use tao_log::*;
use log::{Log, Record, Metadata};

use parking_lot::{ReentrantMutex, ReentrantMutexGuard};

#[cfg(feature = "std")]
use log::set_boxed_logger;

#[cfg(not(feature = "std"))]
fn set_boxed_logger(logger: Box<Log>) -> Result<(), log::SetLoggerError> {
    log::set_logger(Box::leak(logger))
}

struct State {
    last_log: ReentrantMutex<RefCell<Option<String>>>,
}

impl State {
    fn take(&self) -> Option<String> {
        self.last_log.lock().replace(None)
    }
    fn lock(&self) -> ReentrantMutexGuard<RefCell<Option<String>>> {
        self.last_log.lock()
    }
}

struct Logger(Arc<State>);

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let msg = format!("{}", record.args());
        self.0.last_log.lock().replace(Some(msg));
        eprintln!("{:5} {}", record.level(), record.args());
    }

    fn flush(&self) {
        eprintln!("flushed!");
    }
}

fn test_logger() -> Arc<State> {
    use log::LevelFilter;
    static TEST_LOG_INIT: Once = Once::new();
    static mut LOG_STATE: Option<Arc<State>> = None;
    TEST_LOG_INIT.call_once(|| {
        let s = Arc::new(State {
            last_log: ReentrantMutex::new(RefCell::new(None))
        });
        unsafe { LOG_STATE = Some(s.clone()); }
        set_boxed_logger(Box::new(Logger(s))).unwrap();
        log::set_max_level(LevelFilter::Debug);
    });
    unsafe { LOG_STATE.as_ref().unwrap().clone() }
}

struct StaticMsgCheck(Arc<State>);

impl Drop for StaticMsgCheck {
    fn drop(&mut self) {
        eprintln!("checker drop called!");
        assert_eq!(self.0.take(), Some("static fatal msg".to_owned()));
    }
}

#[test]
#[should_panic]
fn fatal_static_msg() {
    let s1 = test_logger();
    let s2 = s1.clone();
    let _test_guard = s1.lock();
    let _fr = StaticMsgCheck(s2);
    fatal!("static fatal msg");
}

struct FormatMsgCheck(Arc<State>);

impl Drop for FormatMsgCheck {
    fn drop(&mut self) {
        eprintln!("checker drop called!");
        assert_eq!(self.0.take(), Some("fmt fatal msg".to_owned()));
    }
}

#[test]
#[should_panic]
fn fatal_format_msg() {
    let s1 = test_logger();
    let s2 = s1.clone();
    let _test_guard = s1.lock();
    let _fr = FormatMsgCheck(s2);
    fatal!(target: "grim", "fmt {} msg", "fatal".to_owned());
}
