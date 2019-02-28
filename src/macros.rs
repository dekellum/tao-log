// Copyright Ⓒ 2019 David Kellum
//
// Beyond the macro public re-exports found here, the _-v_ macros were
// co-developed with https://github.com/rust-lang-nursery/log/pull/316 (by the
// same author) and some other support macros are adopted from log 0.4.6.
//
// All original work is offered under the same Apache 2.0 or MIT licenses and
// is:
// Copyright Ⓒ 2015 The Rust Project Developers

/// Log an expression and its value at any specified level.
///
/// Logs with the optional or default (module path of use) target, specified
/// `Level`, optional prefix, and optional or default (`"{:?}"`) value format
/// string, and a single expression. The expression argument is evaluated
/// exactly once, regardless of if the logging level is enabled, and its
/// value is returned from the macro. This is normally only used through the
/// _-v_ macros like `debugv!` or `tracev!`.
///
/// Note that the value is moved and then returned. If the type does not
/// implement `Copy`, ownership may be retained by borrowing by reference
/// e.g. `debugv!(&expr)`.
///
/// # Examples
///
/// ```rust
/// use log::Level;
/// use tao_log::*;
///
/// #[derive(Debug)]
/// struct Point { x: f32, y: f32 }
///
/// fn circle(center: &Point, radius: f32) { /*...*/ }
///
/// # fn main() {
/// let center = Point { x: 3.234, y: -1.223 };
///
/// circle(logv!(Level::Trace, &center), 7.3);
/// //     ^-- trace level message: "&center → Point { x: 3.234, y: -1.223 }"
/// circle(tracev!(&center), 8.0);
/// //     ^-- trace level message: "&center → Point { x: 3.234, y: -1.223 }"
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! logv {
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => (
        __logv!($lvl, target: $target, $($arg)+)
    );
    ($lvl:expr, $($arg:tt)+) => (
        __logv!($lvl, target: __log_module_path!(), $($arg)+)
    );
}

/// Log an expression at the error level, returning the value.
#[macro_export(local_inner_macros)]
macro_rules! errorv {
    ($($arg:tt)+) => (__logv!($crate::Level::Error, $($arg)+))
}

/// Log an expression at the warn level, returning the value.
#[macro_export(local_inner_macros)]
macro_rules! warnv {
    ($($arg:tt)+) => (__logv!($crate::Level::Warn, $($arg)+))
}

/// Log an expression at the info level, returning the value.
#[macro_export(local_inner_macros)]
macro_rules! infov {
    ($($arg:tt)+) => (__logv!($crate::Level::Info, $($arg)+))
}

/// Log an expression at the debug level, returning the value.
#[macro_export(local_inner_macros)]
macro_rules! debugv {
    ($($arg:tt)+) => (__logv!($crate::Level::Debug, $($arg)+))
}

/// Log an expression at the trace level, returning the value.
#[macro_export(local_inner_macros)]
macro_rules! tracev {
    ($($arg:tt)+) => (__logv!($crate::Level::Trace, $($arg)+))
}

// Helper macro for the -v macros, handling the permutations of optional
// parameters. Note: The required level paremeter is first here for
// convenience of internal use with variable-args.
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __logv {
    ($lvl:expr, target: $tgt:expr, $pre:expr, $vfmt:expr, $val:expr) => (
        __logv_eval!($tgt, $lvl, $pre, $vfmt, $val)
    );
    ($lvl:expr, target: $tgt:expr, $pre:expr, $val:expr) => (
        __logv_eval!($tgt, $lvl, $pre, "{:?}", $val)
    );
    ($lvl:expr, target: $tgt:expr, $val:expr) => (
        __logv_eval!($tgt, $lvl, "", "{:?}", $val)
    );
    ($lvl:expr, $pre:expr, $vfmt:expr, $val:expr) => (
        __logv_eval!(__log_module_path!(), $lvl, $pre, $vfmt, $val)
    );
    ($lvl:expr, $pre:expr, $val:expr) => (
        __logv_eval!(__log_module_path!(), $lvl, $pre, "{:?}", $val)
    );
    ($lvl:expr, $val:expr) => (
        __logv_eval!(__log_module_path!(), $lvl, "", "{:?}", $val)
    );
}

// Inner helper macro for __logv. Evaluates expression exactly once, moves
// value and returns it.
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __logv_eval {
    ($tgt:expr, $lvl:expr, $pre:expr, $vfmt:expr, $exp:expr) => (
        match $exp {
            vt => {
                let pre: &str = $pre;
                let sep = if pre.is_empty() { "" } else { " " };
                $crate::log!(target: $tgt,
                     $lvl,
                     __log_concat!("{}{}{} → ", $vfmt),
                     pre,
                     sep,
                     __log_stringify!($exp),
                     &vt);
                vt
            }
        }
    )
}

// As a workaround for rustc < 1.30, provide thse local wrappers for the above
// macros with local_inner_macros.

#[doc(hidden)]
#[macro_export]
macro_rules! __log_module_path {
    () => {
        module_path!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_stringify {
    ($($args:tt)*) => {
        stringify!($($args)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_concat {
    ($($args:tt)*) => {
        concat!($($args)*)
    };
}
