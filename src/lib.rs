//! A lightweight zero-heap logging library
//!
//! Provides a [`Level`] type representing a log severity, four built-in levels
//! ([`Level::INFO`], [`Level::WARN`], [`Level::ERROR`], [`Level::DEBUG`]), and
//! convenience macros ([`info!`], [`warn!`], [`error!`], [`debug!`]) that mirror
//! the standard `format!` API.
//!
//! Output is written to **stdout** for non-error levels and **stderr** for error
//! levels. Each line is formatted as:
//!
//! ```text
//! [<tag>] <PREFIX> : <message>   # when a tag is supplied
//! <PREFIX> : <message>           # without a tag
//! ```
//!
//! The prefix and message are styled with [`owo_colors`] according to the level.
//! The `debug!` macro compiles to a no-op in release builds.

use std::{fmt, io::{ self, Write }};
use owo_colors::Style;

/// A log level that controls the prefix label, colors, and output stream.
///
/// You can use the four built-in constants ([`Level::INFO`], [`Level::WARN`],
/// [`Level::ERROR`], [`Level::DEBUG`]) or construct a custom level with
/// [`Level::new`] / [`Level::default`].
#[derive(Clone, Copy)]
pub struct Level {
    /// Single-character label printed before every message (e.g. `"I"`, `"W"`).
    prefix: &'static str,
    /// Style applied to [`prefix`](Level::prefix).
    prefix_style: Style,
    /// Style applied to the message body.
    text_style: Style,
    /// When `true`, output goes to stderr instead of stdout.
    is_error: bool,
}

impl Level {
    /// Creates a new `Level` with fully independent prefix and text styles.
    ///
    /// # Arguments
    ///
    /// * `prefix` – Short label shown at the start of every log line.
    /// * `prefix_style` – [`owo_colors::Style`] applied to `prefix`.
    /// * `text_style` – [`owo_colors::Style`] applied to the message body.
    /// * `is_error` – If `true`, messages are written to stderr; otherwise stdout.
    pub const fn new(
        prefix: &'static str,
        prefix_style: Style,
        text_style: Style,
        is_error: bool
    ) -> Self {
        Self {
            prefix,
            prefix_style,
            text_style,
            is_error,
        }
    }

    /// Creates a new `Level` where the prefix style is a **bold** variant of
    /// `text_style`.
    ///
    /// This is a convenience constructor used by the built-in level constants.
    ///
    /// # Arguments
    ///
    /// * `prefix` – Short label shown at the start of every log line.
    /// * `text_style` – [`owo_colors::Style`] applied to the message body; the
    ///   prefix receives the same style with bold applied on top.
    /// * `is_error` – If `true`, messages are written to stderr; otherwise stdout.
    pub const fn default(
        prefix: &'static str,
        text_style: Style,
        is_error: bool
    ) -> Self {
        Self { prefix, prefix_style: text_style.bold(), text_style, is_error }
    }

    /// Informational messages. Prefix `I`, green, writes to stdout.
    pub const INFO: Self = Self::default("I", Style::new().green(), false);
    /// Warning messages. Prefix `W`, yellow, writes to stdout.
    pub const WARN: Self = Self::default("W", Style::new().yellow(), false);
    /// Error messages. Prefix `E`, red, writes to **stderr**.
    pub const ERROR: Self = Self::default("E", Style::new().red(), true);
    /// Debug messages. Prefix `D`, cyan, writes to stdout.
    ///
    /// When using the [`debug!`] macro, this level is a no-op in release builds.
    pub const DEBUG: Self = Self::default("D", Style::new().cyan(), false);

    /// Prints a formatted message with an optional tag prefix.
    ///
    /// Output format:
    /// * With tag: `[<tag>] <PREFIX> : <text>`
    /// * Without tag: `<PREFIX> : <text>`
    ///
    /// Writes to stderr if [`is_error`](Level::is_error) is `true`, stdout otherwise.
    /// The write is performed on the locked handle to prevent interleaving in
    /// multi-threaded contexts.
    ///
    /// # Arguments
    ///
    /// * `tag` – Optional context label (e.g. a module or component name).
    /// * `text` – Pre-formatted [`fmt::Arguments`], typically produced by
    ///   [`format_args!`].
    pub fn print_tagged(&self, tag: Option<&str>, text: fmt::Arguments<'_>) {
        let (stderr, stdout);
        let handle: &mut dyn Write = if self.is_error {
            stderr = io::stderr();
            &mut stderr.lock()
        } else {
            stdout = io::stdout();
            &mut stdout.lock()
        };

        let prefix = self.prefix_style.style(self.prefix);
        let body = self.text_style.style(&text);

        macro_rules! write_log {
            ($tag_fmt:expr, $tag_val:expr) => {
                writeln!(handle, concat!($tag_fmt, "{} : {}"), $tag_val, prefix, body)
            };
            ($tag_fmt:expr) => {
                writeln!(handle, concat!($tag_fmt, "{} : {}"), prefix, body)
            };
        }

        let _ = match tag {
            Some(t) => write_log!("[{}] ", t),
            None => write_log!(""),
        };
    }

    /// Prints a formatted message without a tag.
    ///
    /// Equivalent to calling [`print_tagged`](Level::print_tagged) with
    /// `tag: None`.
    ///
    /// # Arguments
    ///
    /// * `text` – Pre-formatted [`fmt::Arguments`], typically produced by
    ///   [`format_args!`].
    pub fn print(&self, text: fmt::Arguments<'_>) {
        self.print_tagged(None, text);
    }
}

/// Logs a message at the given `level` with an optional `tag`.
///
/// Prefer the [`info!`], [`warn!`], [`error!`], and [`debug!`] macros for
/// typical use. This free function is useful when the level is determined
/// dynamically at runtime.
///
/// # Arguments
///
/// * `level` – The [`Level`] to log at.
/// * `tag` – Optional context label.
/// * `text` – Pre-formatted [`fmt::Arguments`].
pub fn log_tagged(level: Level, tag: Option<&str>, text: fmt::Arguments<'_>) {
    level.print_tagged(tag, text);
}

/// Logs a message at the given `level` without a tag.
///
/// Prefer the [`info!`], [`warn!`], [`error!`], and [`debug!`] macros for
/// typical use. This free function is useful when the level is determined
/// dynamically at runtime.
///
/// # Arguments
///
/// * `level` – The [`Level`] to log at.
/// * `text` – Pre-formatted [`fmt::Arguments`].
pub fn log(level: Level, text: fmt::Arguments<'_>) {
    level.print_tagged(None, text);
}

/// Logs an informational message using [`Level::INFO`].
///
/// Accepts the same syntax as [`format!`]. An optional `tag:` argument places
/// a context label at the start of the line.
///
/// # Examples
///
/// ```
/// info!("server started on port {}", 8080);
/// info!(tag: "net", "connected to {}", addr);
/// ```
#[macro_export]
macro_rules! info {
    (tag: $tag:expr, $($arg:tt)*) => { $crate::Level::INFO.print_tagged(Some($tag), format_args!($($arg)*)); };
    ($($arg:tt)*) => { $crate::Level::INFO.print_tagged(None, format_args!($($arg)*)); };
}

/// Logs a warning message using [`Level::WARN`].
///
/// Accepts the same syntax as [`format!`]. An optional `tag:` argument places
/// a context label at the start of the line.
///
/// # Examples
///
/// ```
/// warn!("retrying request, attempt {}", attempt);
/// warn!(tag: "cfg", "unknown key {:?}, ignoring", key);
/// ```
#[macro_export]
macro_rules! warn {
    (tag: $tag:expr, $($arg:tt)*) => { $crate::Level::WARN.print_tagged(Some($tag), format_args!($($arg)*)); };
    ($($arg:tt)*) => { $crate::Level::WARN.print_tagged(None, format_args!($($arg)*)); };
}

/// Logs an error message using [`Level::ERROR`].
///
/// Output goes to **stderr**. Accepts the same syntax as [`format!`]. An
/// optional `tag:` argument places a context label at the start of the line.
///
/// # Examples
///
/// ```
/// error!("failed to open file: {}", err);
/// error!(tag: "db", "query failed: {}", err);
/// ```
#[macro_export]
macro_rules! error {
    (tag: $tag:expr, $($arg:tt)*) => { $crate::Level::ERROR.print_tagged(Some($tag), format_args!($($arg)*)); };
    ($($arg:tt)*) => { $crate::Level::ERROR.print_tagged(None, format_args!($($arg)*)); };
}

/// Logs a debug message using [`Level::DEBUG`].
///
/// **Compiled out entirely in release builds** (when `debug_assertions` is
/// disabled), so there is no runtime cost in production. Accepts the same
/// syntax as [`format!`]. An optional `tag:` argument places a context label
/// at the start of the line.
///
/// # Examples
///
/// ```
/// debug!("parsed {} tokens", count);
/// debug!(tag: "parser", "state = {:?}", state);
/// ```
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    (tag: $tag:expr, $($arg:tt)*) => { $crate::Level::DEBUG.print_tagged(Some($tag), format_args!($($arg)*)); };
    ($($arg:tt)*) => { $crate::Level::DEBUG.print_tagged(None, format_args!($($arg)*)); };
}

/// No-op version of [`debug!`] used in release builds.
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    (tag: $tag:expr, $($arg:tt)*) => { };
    ($($arg:tt)*) => { };
}

