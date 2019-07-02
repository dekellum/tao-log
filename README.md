# tao-log

[![Crates.io](https://img.shields.io/crates/v/tao-log.svg?maxAge=3600)](https://crates.io/crates/tao-log)
[![Rustdoc](https://docs.rs/tao-log/badge.svg)](https://docs.rs/tao-log)
[![Travis CI Build](https://travis-ci.org/dekellum/tao-log.svg?branch=master)](https://travis-ci.org/dekellum/tao-log)
[![Appveyor CI Build](https://ci.appveyor.com/api/projects/status/iapsfe9s6sre3f0u/branch/master?svg=true)](https://ci.appveyor.com/project/dekellum/tao-log)
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

MSRV := 1.31.0

The crate will fail fast on any lower rustc (via a build.rs version
check) and is also CI tested on this version, included both 2015 and 2018 edition
external macro imports. A PR would be considered to backport the project to
earlier rust versions, possibly as far back as 1.16.0 (_log_'s current MSRV).
Or consider lobbying for the inclusion of this feature in _log_ itself.

With rustc 1.32.0+, one (but not more than one) trailing comma is properly
accepted in _-v_ macro calls.  On 1.31.0, multiple trailing commas are
erroneously accepted.

Compile time errors for misuse of _-v_ macros (e.g. 0 arguments, 4 arguments,
non-literal prefix argument, missing format specifier, etc.) are tested and
verified on rustc 1.35.0 and nightly, currently. Earlier rustc versions may
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
