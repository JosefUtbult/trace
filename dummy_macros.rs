// These macros can be used if a library has a feature `trace` that includes the trace macros. If
// this is disabled, dummy macros are generated. This presumes that std is enabled during tests.
// If not, remove the std::print parts and just leave the macros empty

#[cfg(feature = "trace")]
pub use trace_dep::*; // Or whatever you call the trace dependency

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        #[cfg(test)]
        std::print!($($arg)*);
    };
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! trace_once {
    ($($arg:tt)*) => {
        #[cfg(test)]
        std::print!($($arg)*);
    };
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! traceln {
    ($($arg:tt)*) => {
        #[cfg(test)]
        std::println!($($arg)*);
    };
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! traceln_once {
    ($($arg:tt)*) => {
        #[cfg(test)]
        std::println!($($arg)*);
    };
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! trace_debug {
    ($($arg:tt)*) => {
        #[cfg(test)]
        std::println!($($arg)*);
    };
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! trace_debug_once {
    ($($arg:tt)*) => {
        #[cfg(test)]
        std::println!($($arg)*);
    };
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! trace_info {
    ($($arg:tt)*) => {
        #[cfg(test)]
        std::println!($($arg)*);
    };
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! trace_info_once {
    ($($arg:tt)*) => {
        #[cfg(test)]
        std::println!($($arg)*);
    };
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! trace_warning {
    ($($arg:tt)*) => {
        #[cfg(test)]
        std::println!($($arg)*);
    };
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! trace_warning_once {
    ($($arg:tt)*) => {
        #[cfg(test)]
        std::println!($($arg)*);
    };
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! trace_error {
    ($($arg:tt)*) => {
        #[cfg(test)]
        std::println!($($arg)*);
    };
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! trace_error_once {
    ($($arg:tt)*) => {
        #[cfg(test)]
        std::println!($($arg)*);
    };
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! trace_panic {
    ($($arg:tt)*) => {
        #[cfg(test)]
        std::println!($($arg)*);
    };
}
