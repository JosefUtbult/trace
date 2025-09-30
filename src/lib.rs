#![cfg_attr(not(feature = "std"), no_std)]

//! Simple Rust trace implementation intended for `no_std` targets. Creates a
//! `TraceHandler` trait that can be used to setup a global trace handler. This
//! handler is then used by various trace macros.

mod tests;

use core::{
    cell::RefCell,
    fmt::{self, Write},
    option::Option::{self, None, Some},
    panic,
};

use critical_section::{Mutex, with as critical};

#[cfg(feature = "std")]
extern crate std;

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
            buffer: self.buffer.clone()
        }
    }
}

/// A TraceHandler is any entity setup to handle tracing. This handler will be used in a static
/// context, for access from any part of the system
pub trait TraceHandler: Send + Sync {
    fn trace_write(&self, msg: &str);
}

pub type TraceHandlerReference = &'static dyn TraceHandler;
struct TraceHandlerReferenceContainer(Mutex<RefCell<Option<TraceHandlerReference>>>);

static TRACE_HANDLER: TraceHandlerReferenceContainer = TraceHandlerReferenceContainer::new();

impl TraceHandlerReferenceContainer {
    const fn new() -> Self {
        Self(Mutex::new(RefCell::new(None)))
    }

    fn setup(&self, handler: TraceHandlerReference) {
        critical(|cs| {
            let _ = self.0.borrow(cs).borrow_mut().insert(handler);
        })
    }

    fn cleanup(&self) {
        critical(|cs| {
            let _ = self.0.borrow(cs).borrow_mut().take();
        })
    }

    #[allow(dead_code)] // Only used in testing
    fn is_some(&self) -> bool {
        critical(|cs| self.0.borrow(cs).borrow().is_some())
    }

    /// Write a message string the trace handler
    fn write(&self, msg: &TraceString) {
        critical(|cs| {
            let mut data_ref = self.0.borrow(cs).borrow_mut();
            if let Some(trace_handler) = data_ref.as_mut() {
                trace_handler.trace_write(msg.to_string());
            } else {
                panic!("Trace handler has not been initialized");
            }
        })
    }

    /// Write a panic message string the trace handler. Should not panic on uninitialized trace
    /// handler
    fn write_panic(&self, msg: &TraceString) {
        critical(|cs| {
            let mut data_ref = self.0.borrow(cs).borrow_mut();
            if let Some(trace_handler) = data_ref.as_mut() {
                trace_handler.trace_write(msg.to_string());
            }
        })
    }
}

pub(crate) fn format(args: fmt::Arguments) -> TraceString {
    let mut res = TraceString::new();
    unsafe {
        fmt::write(&mut res, args).unwrap_unchecked();
    }
    res
}

/// Setup a new trace handler
pub fn trace_setup(trace_handler: TraceHandlerReference) {
    TRACE_HANDLER.setup(trace_handler);
}

/// Remove the current trace handler
pub fn trace_cleanup() {
    TRACE_HANDLER.cleanup();
}

/// Check if the trace handler has been initialized
pub fn trace_is_some() -> bool {
    TRACE_HANDLER.is_some()
}

/// Write a message string to the trace handler
pub fn trace_write(args: fmt::Arguments) {
    TRACE_HANDLER.write(&format(args));
}

/// Write a panic message string to the trace handler
pub fn trace_write_panic(args: fmt::Arguments) {
    TRACE_HANDLER.write_panic(&format(args));
}

/// Helper macro to simplify testing. Calls `std::print` during testing if the trace handler isn't
/// set up. Allows for testing code that uses trace functionality without having to setup the trace
/// handler at each test
#[cfg(feature = "std")]
#[macro_export]
macro_rules! call_trace_write {
    ($($arg:tt)*) => {
        #[cfg(test)]
        {
            if $crate::trace_is_some() {
                $crate::trace_write(format_args!($($arg)*));
            } else {
                extern crate std;
                std::print!($($arg)*);
            }
        }
        #[cfg(not(test))]
        $crate::trace_write(format_args!($($arg)*));
    };
}

/// Helper macro to simplify testing. If the `no-std-during-tests` feature is enabled, this version
/// of the macro doesn't try to use `std::print` during testing. The test must in this case setup
/// the trace handler manually.
#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! call_trace_write {
    ($($arg:tt)*) => {
        $crate::trace_write(format_args!($($arg)*));
    };
}

/// Helper macro to simplify testing. Calls `std::print` during testing if the trace handler isn't
/// set up. Allows for testing code that uses trace functionality without having to setup the trace
/// handler at each test
#[cfg(feature = "std")]
#[macro_export]
macro_rules! call_trace_write_panic {
    ($($arg:tt)*) => {
        #[cfg(test)]
        {
            if $crate::trace_is_some() {
                $crate::trace_write_panic(format_args!($($arg)*));
            } else {
                extern crate std;
                std::print!($($arg)*);
            }
        }
        #[cfg(not(test))]
        $crate::trace_write_panic(format_args!($($arg)*));
    };
}

/// Helper macro to simplify testing. If the `no-std-during-tests` feature is enabled, this version
/// of the macro doesn't try to use `std::print` during testing. The test must in this case setup
/// the trace handler manually.
#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! call_trace_write_panic {
    ($($arg:tt)*) => {
        $crate::trace_write_panic(format_args!($($arg)*));
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::call_trace_write!($($arg)*);
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
            $crate::call_trace_write!("\x1b[0m{}\r\n", format_args!($($arg)*));
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
            $crate::call_trace_write!("{}\r\n", format_args!($($arg)*));
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
            $crate::call_trace_write!("\x1b[35mDEBUG: {}\x1b[0m\r\n", format_args!($($arg)*));
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
            $crate::call_trace_write!("DEBUG: {}\r\n", format_args!($($arg)*));
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
            $crate::call_trace_write!("\x1b[32mINFO: {}\x1b[0m\r\n", format_args!($($arg)*));
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
            $crate::call_trace_write!("INFO: {}\r\n", format_args!($($arg)*));
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
            $crate::call_trace_write!("\x1b[33mWARNING: {}\x1b[0m\r\n", format_args!($($arg)*));
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
            $crate::call_trace_write!("WARNING: {}\r\n", format_args!($($arg)*));
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
            $crate::call_trace_write!("\x1b[31mERROR: {}\x1b[0m\r\n", format_args!($($arg)*));
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
            $crate::call_trace_write!("ERROR: {}\r\n", format_args!($($arg)*));
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
            $crate::call_trace_write_panic!("\x1b[31mPANIC: {}\x1b[0m\r\n", format_args!($($arg)*));
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
            $crate::call_trace_write_panic!("PANIC: {}\r\n", format_args!($($arg)*));
        }
    };
}
