use std::{fmt, io::{ self, Write }};
use owo_colors::Style;

#[derive(Clone, Copy)]
pub struct Level {                                                                                                                 
    prefix: &'static str,
    prefix_style: Style,                                                                                                           
    text_style: Style,
    is_error: bool,
}

impl Level {
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

    pub const fn default(
        prefix: &'static str,
        text_style: Style,
        is_error: bool
    ) -> Self {
        Self { prefix, prefix_style: text_style.bold(), text_style, is_error }
    }

    pub const INFO: Self = Self::default("I", Style::new().green(), false);
    pub const WARN: Self = Self::default("W", Style::new().yellow(), false);
    pub const ERROR: Self = Self::default("E", Style::new().red(), true);
    pub const DEBUG: Self = Self::default("D", Style::new().cyan(), false);

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

    pub fn print(&self, text: fmt::Arguments<'_>) {
        self.print_tagged(None, text);
    }
}

pub fn log_tagged(level: Level, tag: Option<&str>, text: fmt::Arguments<'_>) {
    level.print_tagged(tag, text);
}

pub fn log(level: Level, text: fmt::Arguments<'_>) {
    level.print_tagged(None, text);
}

#[macro_export]
macro_rules! info {
    (tag: $tag:expr, $($arg:tt)*) => { $crate::Level::INFO.print_tagged(Some($tag), format_args!($($arg)*)); };
    ($($arg:tt)*) => { $crate::Level::INFO.print_tagged(None, format_args!($($arg)*)); };
}

#[macro_export]
macro_rules! warn {
    (tag: $tag:expr, $($arg:tt)*) => { $crate::Level::WARN.print_tagged(Some($tag), format_args!($($arg)*)); };
    ($($arg:tt)*) => { $crate::Level::WARN.print_tagged(None, format_args!($($arg)*)); };
}

#[macro_export]
macro_rules! error {
    (tag: $tag:expr, $($arg:tt)*) => { $crate::Level::ERROR.print_tagged(Some($tag), format_args!($($arg)*)); };
    ($($arg:tt)*) => { $crate::Level::ERROR.print_tagged(None, format_args!($($arg)*)); };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    (tag: $tag:expr, $($arg:tt)*) => { $crate::Level::DEBUG.print_tagged(Some($tag), format_args!($($arg)*)); };
    ($($arg:tt)*) => { $crate::Level::DEBUG.print_tagged(None, format_args!($($arg)*)); };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    (tag: $tag:expr, $($arg:tt)*) => { };
    ($($arg:tt)*) => { };
}

