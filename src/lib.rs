#![cfg_attr(not(test), no_std)]

//! Simple Rust trace implementation intended for `no_std` targets. Creates a
//! `TraceHandler` trait that can be used to setup a global trace handler. This
//! handler is then used by various trace macros.

#[cfg(test)]
mod tests;

use core::fmt::{self, Write};

unsafe extern "Rust" {
    fn _on_trace(msg: &str);
}

/// Helper macro to allow a user to define an extern trace_write function
/// with a closure
#[macro_export]
macro_rules! on_trace {
    ($handler:expr) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "Rust" fn _on_trace(msg: &str) {
            $handler(msg)
        }
    };
}

pub(crate) const TRACE_FORMAT_BUFFER_SIZE: usize = 1024;

/// A `TraceString` is a string that can be formatted up to a size of 1024. Anything larger than
/// that will be cut of
pub(crate) struct TraceString {
    pub(crate) length: usize,
    pub(crate) buffer: [u8; TRACE_FORMAT_BUFFER_SIZE],
}

impl TraceString {
    const fn new() -> Self {
        Self {
            length: 0,
            buffer: [0; TRACE_FORMAT_BUFFER_SIZE],
        }
    }

    fn to_string(&self) -> &str {
        core::str::from_utf8(&self.buffer[..self.length]).unwrap()
    }
}

impl Write for TraceString {
    fn write_str(&mut self, string: &str) -> core::fmt::Result {
        // Just cut this part of the string of
        if self.length == TRACE_FORMAT_BUFFER_SIZE {
            return Ok(());
        }

        let max_character_length = usize::min(string.len(), TRACE_FORMAT_BUFFER_SIZE - self.length);
        let source_substring = &string[0..max_character_length];
        let dest_charlist = &mut self.buffer[self.length..self.length + max_character_length];

        dest_charlist.copy_from_slice(source_substring.as_bytes());
        self.length += max_character_length;
        Ok(())
    }
}

// Used for testing
#[cfg(test)]
impl Clone for TraceString {
    fn clone(&self) -> Self {
        Self {
            length: self.length,
            buffer: self.buffer.clone(),
        }
    }
}

pub(crate) fn format(args: fmt::Arguments) -> TraceString {
    let mut res = TraceString::new();
    unsafe {
        fmt::write(&mut res, args).unwrap_unchecked();
    }
    res
}

pub fn trace_format(args: fmt::Arguments) {
    unsafe { _on_trace(format(args).to_string()) };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::trace_format(format_args!($($arg)*));
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
macro_rules! trace_once {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            use core::sync::atomic::{AtomicBool, Ordering::SeqCst, Ordering::Relaxed};
            static HAS_RUN: AtomicBool = AtomicBool::new(false);
            if HAS_RUN.compare_exchange(false, true, SeqCst, Relaxed).is_ok() {
                $crate::trace!($($arg)*);
            }
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[cfg(not(feature = "no-color"))]
#[macro_export]
macro_rules! traceln {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::trace_format(format_args!("\x1b[0m{}\r\n", format_args!($($arg)*)));
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[cfg(feature = "no-color")]
#[macro_export]
macro_rules! traceln {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::trace_format(format_args!("{}\r\n", format_args!($($arg)*)));
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
macro_rules! traceln_once {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            use core::sync::atomic::{AtomicBool, Ordering::SeqCst, Ordering::Relaxed};
            static HAS_RUN: AtomicBool = AtomicBool::new(false);
            if HAS_RUN.compare_exchange(false, true, SeqCst, Relaxed).is_ok() {
                $crate::traceln!($($arg)*);
            }
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[cfg(not(feature = "no-color"))]
#[macro_export]
macro_rules! trace_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::trace_format(format_args!("\x1b[35mDEBUG: {}\x1b[0m\r\n", format_args!($($arg)*)));
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[cfg(feature = "no-color")]
#[macro_export]
macro_rules! trace_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::trace_format(format_args!("DEBUG: {}\r\n", format_args!($($arg)*)));
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
macro_rules! trace_debug_once {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            use core::sync::atomic::{AtomicBool, Ordering::SeqCst, Ordering::Relaxed};
            static HAS_RUN: AtomicBool = AtomicBool::new(false);
            if HAS_RUN.compare_exchange(false, true, SeqCst, Relaxed).is_ok() {
                $crate::trace_debug!($($arg)*);
            }
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[cfg(not(feature = "no-color"))]
#[macro_export]
macro_rules! trace_info {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::trace_format(format_args!("\x1b[32mINFO: {}\x1b[0m\r\n", format_args!($($arg)*)));
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[cfg(feature = "no-color")]
#[macro_export]
macro_rules! trace_info {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::trace_format(format_args!("INFO: {}\r\n", format_args!($($arg)*)));
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
macro_rules! trace_info_once {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            use core::sync::atomic::{AtomicBool, Ordering::SeqCst, Ordering::Relaxed};
            static HAS_RUN: AtomicBool = AtomicBool::new(false);
            if HAS_RUN.compare_exchange(false, true, SeqCst, Relaxed).is_ok() {
                $crate::trace_info!($($arg)*);
            }
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[cfg(not(feature = "no-color"))]
#[macro_export]
macro_rules! trace_warning {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::trace_format(format_args!("\x1b[33mWARNING: {}\x1b[0m\r\n", format_args!($($arg)*)));
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[cfg(feature = "no-color")]
#[macro_export]
macro_rules! trace_warning {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::trace_format(format_args!("WARNING: {}\r\n", format_args!($($arg)*)));
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
macro_rules! trace_warning_once {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            use core::sync::atomic::{AtomicBool, Ordering::SeqCst, Ordering::Relaxed};
            static HAS_RUN: AtomicBool = AtomicBool::new(false);
            if HAS_RUN.compare_exchange(false, true, SeqCst, Relaxed).is_ok() {
                $crate::trace_warning!($($arg)*);
            }
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
#[cfg(not(feature = "no-color"))]
macro_rules! trace_error {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::trace_format(format_args!("\x1b[31mERROR: {}\x1b[0m\r\n", format_args!($($arg)*)));
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
#[cfg(feature = "no-color")]
macro_rules! trace_error {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::trace_format(format_args!("ERROR: {}\r\n", format_args!($($arg)*)));
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
macro_rules! trace_error_once {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            use core::sync::atomic::{AtomicBool, Ordering::SeqCst, Ordering::Relaxed};
            static HAS_RUN: AtomicBool = AtomicBool::new(false);
            if HAS_RUN.compare_exchange(false, true, SeqCst, Relaxed).is_ok() {
                $crate::trace_error!($($arg)*);
            }
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[cfg(not(feature = "no-color"))]
#[macro_export]
macro_rules! trace_panic {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::trace_format(format_args!("\x1b[31mPANIC: {}\x1b[0m\r\n", format_args!($($arg)*)));
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[cfg(feature = "no-color")]
#[macro_export]
macro_rules! trace_panic {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::trace_format(format_args!("PANIC: {}\r\n", format_args!($($arg)*)));
        }
    };
}
