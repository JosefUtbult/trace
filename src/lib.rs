#![no_std]

//! Simple Rust trace implementation intended for `no_std` targets. Creates a
//! `TraceHandler` trait that can be used to setup a global trace handler. This
//! handler is then used by various trace macros.

mod tests;

pub use fixed_string::FixedString;
use fixed_string::FixedStringRef;

use core::{
    cell::RefCell,
    option::Option::{self, None, Some},
    panic,
};

use critical_section::{Mutex, with as critical};

/// A TraceHandler is any entity setup to handle tracing. This handler will be used in a static
/// context, for access from any part of the system
pub trait TraceHandler: Send + Sync {
    fn trace_write(&self, msg: &dyn FixedStringRef);
}

pub type TraceHandlerReference = &'static dyn TraceHandler;
struct TraceHandlerReferenceContainer(Mutex<RefCell<Option<TraceHandlerReference>>>);

static TRACE_HANDLER: TraceHandlerReferenceContainer = TraceHandlerReferenceContainer::new();

/// Full capacity for strings formatted by the trace macros. Any string formatted larger than this
/// will cause a panic
pub const TRACE_FORMAT_BUFFER_SIZE: usize = 1024;

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
    fn write(&self, msg: &dyn FixedStringRef) {
        critical(|cs| {
            let mut data_ref = self.0.borrow(cs).borrow_mut();
            if let Some(trace_handler) = data_ref.as_mut() {
                trace_handler.trace_write(msg);
            } else {
                panic!("Trace handler has not been initialized");
            }
        })
    }

    /// Write a panic message string the trace handler. Should not panic on uninitialized trace
    /// handler
    fn write_panic(&self, msg: &dyn FixedStringRef) {
        critical(|cs| {
            let mut data_ref = self.0.borrow(cs).borrow_mut();
            if let Some(trace_handler) = data_ref.as_mut() {
                trace_handler.trace_write(msg);
            }
        })
    }
}

/// Setup a new trace handler
pub fn trace_setup(trace_handler: TraceHandlerReference) {
    TRACE_HANDLER.setup(trace_handler);
}

/// Remove the current trace handler
pub fn trace_cleanup() {
    TRACE_HANDLER.cleanup();
}

/// Write a message string to the trace handler
pub fn trace_write(msg: &dyn FixedStringRef) {
    TRACE_HANDLER.write(msg);
}

/// Write a panic message string to the trace handler
pub fn trace_write_panic(msg: &dyn FixedStringRef) {
    TRACE_HANDLER.write_panic(msg);
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            let formatted = $crate::FixedString::<{$crate::TRACE_FORMAT_BUFFER_SIZE}>::format(
                format_args!($($arg)*)
            ).unwrap();

            $crate::trace_write(&formatted);
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
                crate::trace!($($arg)*);
            }
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
macro_rules! traceln {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            #[cfg(not(feature = "no-color"))]
            {
                let formatted = $crate::FixedString::<{$crate::TRACE_FORMAT_BUFFER_SIZE}>::format(
                    format_args!(
                        "\x1b[0m{}\r\n",
                        format_args!($($arg)*)
                    )
                ).unwrap();

                $crate::trace_write(&formatted);
            }
            #[cfg(feature = "no-color")]
            {
                let formatted = $crate::FixedString::<{$crate::TRACE_FORMAT_BUFFER_SIZE}>::format(
                    format_args!(
                        "{}\r\n",
                        format_args!($($arg)*)
                    )
                ).unwrap();

                $crate::trace_write(&formatted);
            }
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
                crate::traceln!($($arg)*);
            }
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
macro_rules! trace_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            #[cfg(not(feature = "no-color"))]
            {
                let formatted = $crate::FixedString::<{$crate::TRACE_FORMAT_BUFFER_SIZE}>::format(
                    format_args!(
                        "\x1b[35mDEBUG: {}\x1b[0m\r\n",
                        format_args!($($arg)*)
                    )
                ).unwrap();

                $crate::trace_write(&formatted);
            }
            #[cfg(feature = "no-color")]
            {
                let formatted = $crate::FixedString::<{$crate::TRACE_FORMAT_BUFFER_SIZE}>::format(
                    format_args!(
                        "DEBUG: {}\r\n",
                        format_args!($($arg)*)
                    )
                ).unwrap();

                $crate::trace_write(&formatted);
            }
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
                crate::trace_debug!($($arg)*);
            }
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
macro_rules! trace_info {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            #[cfg(not(feature = "no-color"))]
            {
                let formatted = $crate::FixedString::<{$crate::TRACE_FORMAT_BUFFER_SIZE}>::format(
                    format_args!(
                        "\x1b[32mINFO: {}\x1b[0m\r\n",
                        format_args!($($arg)*)
                    )
                ).unwrap();

                $crate::trace_write(&formatted);
            }
            #[cfg(feature = "no-color")]
            {
                let formatted = $crate::FixedString::<{$crate::TRACE_FORMAT_BUFFER_SIZE}>::format(
                    format_args!(
                        "INFO: {}\r\n",
                        format_args!($($arg)*)
                    )
                ).unwrap();

                $crate::trace_write(&formatted);
            }
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
                crate::trace_info!($($arg)*);
            }
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
macro_rules! trace_warning {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            #[cfg(not(feature = "no-color"))]
            {
                let formatted = $crate::FixedString::<{$crate::TRACE_FORMAT_BUFFER_SIZE}>::format(
                    format_args!(
                        "\x1b[33mWARNING: {}\x1b[0m\r\n",
                        format_args!($($arg)*)
                    )
                ).unwrap();

                $crate::trace_write(&formatted);
            }
            #[cfg(feature = "no-color")]
            {
                let formatted = $crate::FixedString::<{$crate::TRACE_FORMAT_BUFFER_SIZE}>::format(
                    format_args!(
                        "WARNING: {}\r\n",
                        format_args!($($arg)*)
                    )
                ).unwrap();

                $crate::trace_write(&formatted);
            }
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
                crate::trace_warning!($($arg)*);
            }
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
macro_rules! trace_error {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            #[cfg(not(feature = "no-color"))]
            {
                let formatted = $crate::FixedString::<{$crate::TRACE_FORMAT_BUFFER_SIZE}>::format(
                    format_args!(
                        "\x1b[31mERROR: {}\x1b[0m\r\n",
                        format_args!($($arg)*)
                    )
                ).unwrap();

                $crate::trace_write(&formatted);
            }
            #[cfg(feature = "no-color")]
            {
                let formatted = $crate::FixedString::<{$crate::TRACE_FORMAT_BUFFER_SIZE}>::format(
                    format_args!(
                        "ERROR: {}\r\n",
                        format_args!($($arg)*)
                    )
                ).unwrap();

                $crate::trace_write(&formatted);
            }
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
                crate::trace_error!($($arg)*);
            }
        }
    };
}

/// Tracing macro for simplifying the usage of the trace functionality. Will panic if the formatted
/// string is to long
#[macro_export]
macro_rules! trace_panic {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            #[cfg(not(feature = "no-color"))]
            {
                let res = $crate::FixedString::<{$crate::TRACE_FORMAT_BUFFER_SIZE}>::format(
                    format_args!(
                        "\x1b[31mPANIC: {}\x1b[0m\r\n",
                        format_args!($($arg)*)
                    )
                );
                match res {
                    Ok(res) => {
                        $crate::trace_write_panic(&res);
                    },
                    Err(err) => {
                        let res = $crate::FixedString::<{$crate::TRACE_FORMAT_BUFFER_SIZE}>::format(
                            format_args!("\x1b[31mPANIC: Format error {:?}\x1b[0m\r\n", err)
                        );

                        match res {
                            Ok(res) => {
                                $crate::trace_write_panic(&res);
                            },
                            Err(_) => {}
                        }
                    }
                }
            }
            #[cfg(feature = "no-color")]
            {
                let res = $crate::FixedString::<{$crate::TRACE_FORMAT_BUFFER_SIZE}>::format(
                    format_args!(
                        "PANIC: {}\r\n",
                        format_args!($($arg)*)
                    )
                );
                match res {
                    Ok(res) => {
                        $crate::trace_write_panic(&res);
                    },
                    Err(err) => {
                        let res = $crate::FixedString::<{$crate::TRACE_FORMAT_BUFFER_SIZE}>::format(
                            format_args!("PANIC: Format error {:?}\r\n", err)
                        );

                        match res {
                            Ok(res) => {
                                $crate::trace_write_panic(&res);
                            },
                            Err(_) => {}
                        }
                    }
                }
            }
        }
    };
}
