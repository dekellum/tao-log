// Copyright Ⓒ 2019 David Kellum
//
// Documentation here is derivative work from:
//
// * https://github.com/rust-lang-nursery/log/pull/316 (same author)
// * https://github.com/rust-lang-nursery/log
//
// Offered under the same Apache 2.0 or MIT licenses and is:
// Copyright Ⓒ 2015 The Rust Project Developers

//! Extension macros for output to the rust _log_ crate.
//!
//! > <em>“Why write an RFC, when you can just write code?”</em>
//! > — 老子 (“Old Master”), 557 BCE
//!
//! This unenlightened wanderer (游侠) wrote a _log_ crate [PR] and [RFC 317],
//! before published this standalone crate.
//!
//! Libraries and applications can use the _tao-log_ macros for all log output
//! needs. This includes re-exported _log_ crate _formatted logging_ and
//! associated macros, and a set of new “_-v_” suffix macros (e.g. `debugv!`)
//! that provide _inline expression and value_ logging, as a superset of
//! `std::dbg!`. Both sets of macros are given a new overview below, with
//! examples using arbitrary log levels.
//!
//! If the new macros are eventually accepted to the _log_ crate in compatible
//! form, this crate will be re-released with those re-exports, and possibly
//! deprecated after that.
//!
//! Using the rust 2018 edition, all exported macros can be imported like a
//! prelude:
//!
//! ```rust
//! use tao_log::*;
//! ```
//! Or if desired, imported on a per macro basis:
//!
//! ```rust
//! use tao_log::{debugv, debug, error, warn};
//! ```
//!
//! ### Formatted logging
//!
//! The original macros for each supported level, from highest to lowest
//! priority (or lowest to highest verbosity) include: `error!`, `warn!`,
//! `info!`, `debug!` and `trace!`. These are passed a literal format string,
//! supporting the same syntax as `println!`, via [`std::fmt`]:
//!
//! ```rust
//! use tao_log::*;
//!
//! info!("Landing gear retracted");
//! let altitude = 3000;
//! let target = 10500;
//! debug!("Altitude target: {}, current: {}", target, altitude);
//! ```
//!
//! If for example, the configured maximum logging level is `Info`, then the
//! above `debug!` statement does not log, and the cost of formatting the
//! message string is avoided.
//!
//! ### Testing for output
//!
//! The [`log_enabled!`](macro.log_enabled.html) macro may be used to
//! explicitly test if logging is enabled, and may be useful to avoid
//! expensive computations used only for logging.
//!
//! ```rust
//! # struct Foo;
//! # impl Foo {
//! #     fn volume(&self) -> f32 { 0.1 }
//! #     fn mass(&self) -> f32 { 0.2 }
//! # }
//! # fn analyze(a: u32) -> Foo { Foo }
//! use tao_log::*;
//! use tao_log::log::Level;
//!
//! # fn main() {
//! # let asteroid = 1;
//! if log_enabled!(Level::Debug) {
//!     let e = analyze(asteroid); // expensive!
//!     debug!("Asteroid volume: {}, mass: {}", e.volume(), e.mass());
//! }
//! # }
//! ```
//!
//! ### Inline expression and value logging
//!
//! The _-v_ macros support inline expression and value logging, as a superset
//! of the [`std::dbg!`] macro, for use with the logging system. A _single_
//! expression argument is evaluated exactly once, regardless of if the
//! logging level is enabled, and its value is returned from the macro. Given
//! this code as a starting point:
//!
//! ```rust
//! use std::time::{Duration, Instant};
//! # let deadline = Instant::now() + Duration::new(120, 0);
//!
//! let remaining = deadline - Instant::now();
//! ```
//!
//! A _-v_ macro may be inserted inline around any expression or
//! sub-expression:
//!
//! ```rust
//! use std::time::{Duration, Instant};
//! use tao_log::*;
//! # let deadline = Instant::now() + Duration::new(0, 950_000);
//!
//! let remaining = debugv!(deadline - Instant::now());
//! //               ^-- debug log: deadline - Instant::now() → 950µs
//! debugv!(remaining);
//! // or            ^-- debug log: remaining → 950µs
//! ```
//!
//! Note that the value of the expression is moved and then returned. If the
//! type does not implement `Copy`, ownership may be retained by borrowing by
//! reference, e.g. `debugv!(&expr)`.
//!
//! The _default_ format string for the _-v_ macros is `"{} → {:?}"`, where
//! the `stringify!`-ed expression and resulting value are passed, in that
//! order.  If the log record is not filtered out, the `Debug` implementation
//! for the type of the given expression value is used (`"{:?}"`).
//!
//! The log record can be customized via two optional parameters: a message
//! prefix string, and a format specifier for the value. Note that the former
//! is required, if passing the later:
//!
//! ```rust
//! use tao_log::*;
//!
//! let i = 32;
//! infov!(i);
//! infov!("", "{:?}", i);       // equivalent to above
//! // ^------------------------ info log: i → 32
//! infov!("index", i);          // prefix for additional context
//! infov!("index", "{}", i);    // use `Display` format for value
//! // ^------------------------ info log: index i → 32
//! infov!("index", "{:#x}", i); // hexadecimal format value
//! // ^------------------------ info log: index i → 0x20
//! infov!("index", "{:#?}", i); // pretty multi-line format (for structs)
//! ```
//!
//! ### Specifying the logging target
//!
//! For _all_ logging macros, the _target_ defaults to the module path of the
//! current location of use, but may be overridden with the `target:` marker
//! and string literal as the first argument:
//!
//! ```rust
//! # fn stats() -> i32 { 33 }
//! # fn main() {
//! use tao_log::*;
//! use tao_log::log::Level;
//!
//! let i = 33;
//! let j = debugv!(target: "maths", "halved", (i-1) / 2);
//! # assert_eq!(j, 16);
//!
//! if log_enabled!(target: "special", Level::Info) {
//!     info!(target: "special", "{}", stats());
//! }
//! # }
//! ```
//!
//! [PR]: https://github.com/rust-lang-nursery/log/pull/316
//! [RFC 317]: https://github.com/rust-lang-nursery/log/pull/317
//! [`std::fmt`]: https://doc.rust-lang.org/stable/std/fmt/index.html
//! [`std::dbg!`]: https://doc.rust-lang.org/std/macro.dbg.html

#![doc(html_logo_url = "http://gravitext.com/svg/yin_yang.svg")]

pub use ::log;

pub use log::{debug, error, info, log, log_enabled, trace, warn};

#[macro_use] mod macros;
