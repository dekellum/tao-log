static PACKAGE: &str = "tao-log";
static MSRV: &str = "1.31.0";
static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    static M_V: &str = "minimum supported rust version (MSRV)";

    match version_check::is_min_version(MSRV) {
        Some((true, actual_v)) => {
            eprintln!(
                "{} v{} {} test passed: {} (actual rustc)",
                PACKAGE, VERSION, M_V, actual_v);
        }
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
            println!("cargo:rustc-cfg=trailing_comma");
        }
        _ => {}
    }
}
