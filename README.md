# tao-log

[![Rustdoc](https://docs.rs/tao-log/badge.svg)](https://docs.rs/tao-log)
[![Change Log](https://img.shields.io/crates/v/tao-log.svg?maxAge=3600&label=change%20log&color=9cf)](https://github.com/dekellum/tao-log/blob/main/CHANGELOG.md)
[![Crates.io](https://img.shields.io/crates/v/tao-log.svg?maxAge=3600)](https://crates.io/crates/tao-log)
[![CI Status](https://github.com/dekellum/tao-log/workflows/CI/badge.svg?branch=main)](https://github.com/dekellum/tao-log/actions?query=workflow%3ACI)
[![deps status](https://deps.rs/repo/github/dekellum/tao-log/status.svg)](https://deps.rs/repo/github/dekellum/tao-log)

## Extension macros for output to the rust _log_ crate

Most notably this includes a set of _-v_ suffix macros
(e.g. `debugv!`) that provide a superset of the `std::dbg!` feature.

> <em>“Why write an RFC, when you can just write code?”</em>
> — 老子 (“Old Master”), 557 BCE

This unenlightened wanderer (游侠) wrote a _log_ crate [PR] and
[RFC 317: _Inline Expression and Value Logging_][RFC 317], before publishing
the work as this standalone crate.

See the [rustdoc](https://docs.rs/tao-log) for usage details.

## Minimum supported rust version

MSRV := 1.32.0

The crate will fail fast on any lower rustc (via a build.rs version
check) and is also CI tested on this version, included both 2015 and 2018 edition
external macro imports. A PR would be considered to backport the project to
earlier rust versions, possibly as far back as 1.16.0 (_log_'s current MSRV).
Or consider lobbying for the inclusion of this feature in _log_ itself.

Compile time errors for misuse of _-v_ macros (e.g. 0 arguments, 4 arguments,
non-literal prefix argument, missing format specifier, etc.) are tested and
verified (see ./test_compile_errors). Rustc versions prior to 1.39.0 may
produce less clear errors.

## License

This project is dual licensed under either of following:

* The Apache License, version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)

* The MIT License ([LICENSE-MIT](LICENSE-MIT)
  or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in tao-log by you, as defined by the Apache License, shall be
dual licensed as above, without any additional terms or conditions.

[log]: https://docs.rs/crate/log
[PR]: https://github.com/rust-lang-nursery/log/pull/316
[RFC 317]: https://github.com/rust-lang-nursery/log/pull/317
