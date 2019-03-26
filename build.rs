extern crate version_check;

fn main() {
    match version_check::is_min_version("1.31.0") {
        Some((true, actual_v)) => {
            eprintln!("tao-log MSRV test passed: {} (actual)", actual_v)
        }
        Some((false, actual_v)) => {
            panic!("tao-log MSRV is 1.31.0 > {} (this rustc)", actual_v);
        }
        None => {
            panic!("couldn't query version info from rustc");
        }
    }
}
