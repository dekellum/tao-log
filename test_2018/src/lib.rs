//! Simple testing to confirm 2018-edition style import works, in an external
//! project, for all public macros. No logger is configured for output. No-op
//! is fine for this purpose.

// Exhaustively list all directly used macros, to test without any other
// hidden helper macros in scope.
#[cfg(test)] use tao_log::{
    fatal,
    log, log_enabled,
    trace, debug, info, warn, error,
    logv,
    tracev, debugv, infov, warnv, errorv,
};

#[test]
fn test_2018_log_macros() {
    let i = 32;
    if log_enabled!(target: "special", log::Level::Trace) {
        trace!(target: "special", "{}", i);
    }
    debug!("{:?}", i);
    info!("info");
    warn!("warn");
    error!("error");
    log!(log::Level::Error, "error");
}

#[test]
fn test_2018_logv_macros() {
    let v = true;
    logv!(target: "special", log::Level::Trace, v);
    tracev!(v);
    debugv!(v);
    infov!("prefix", v);
    warnv!("prefix", "{:?}", v);
    assert!(errorv!(v));
}

#[test]
#[should_panic]
fn test_2018_fatal_static_msg() {
    fatal!("static fatal msg");
}

#[test]
#[should_panic]
fn test_2018_fatal_format_msg() {
    fatal!("fmt {}", "failing");
}
