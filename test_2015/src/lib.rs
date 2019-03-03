//! Simple testing to confirm 2015-edition style macro_use works, in an
//! external project, for all public macros. No logger is configured for
//! output. No-op is fine for this purpose.

#[cfg(test)] #[macro_use] extern crate tao_log;
#[cfg(test)] use tao_log::log::Level;

#[test]
fn test_2015_log_macros() {
    let i = 32;
    if log_enabled!(target: "special", Level::Trace) {
        trace!(target: "special", "{}", i);
    }
    debug!("{:?}", i);
    info!("info");
    warn!("warn");
    error!("error");
    log!(Level::Error, "error");
}

#[test]
fn test_2015_logv_macros() {
    let v = true;
    logv!(target: "special", Level::Trace, v);
    tracev!(v);
    debugv!(v);
    infov!(v);
    warnv!(v);
    assert!(errorv!(v));
}
