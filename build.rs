static PACKAGE: &'static str = "tao-log";
static MSRV: &'static str = "1.31.0";

static VERSION: &'static str = env!("CARGO_PKG_VERSION");

extern crate version_check;

fn main() {
    static M_V: &'static str = "minimum supported rust version (MSRV)";

    match version_check::is_min_version(MSRV) {
        Some((true, _actual_v)) => {}
        Some((false, actual_v)) => {
            panic!(
                "{} v{} {} is {} > {} (this rustc)",
                PACKAGE, VERSION, M_V, MSRV, actual_v);
        }
        None => {
            panic!(
                "{} v{}: couldn't query {} from rustc",
                PACKAGE, VERSION, M_V);
        }
    }

    match version_check::is_min_version("1.32.0") {
        Some((true, _)) => {
            println!("cargo:rustc-cfg=tao_log_trailing_comma");
        }
        _ => {}
    }
}
