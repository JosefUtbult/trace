use core::cell::RefCell;

use critical_section::{Mutex, with as critical};
use fixed_string::{FixedString, FixedStringRef};

use crate::{
    TraceHandler, trace, trace_cleanup, trace_debug, trace_debug_once, trace_error,
    trace_error_once, trace_info, trace_info_once, trace_once, trace_panic, trace_setup,
    trace_warning, trace_warning_once, traceln, traceln_once,
};

type TraceBuffer = Mutex<RefCell<FixedString<256>>>;

const fn get_trace_buffer() -> TraceBuffer {
    Mutex::new(RefCell::new(FixedString::new()))
}

struct TestTraceHandler {
    buffer: TraceBuffer,
}

impl TestTraceHandler {
    const fn new() -> Self {
        Self {
            buffer: get_trace_buffer(),
        }
    }

    fn get_result(&self) -> FixedString<256> {
        critical(|cs| self.buffer.borrow(cs).borrow().clone())
    }

    fn reset(&self) {
        let res = critical(|cs| {
            self.buffer.borrow(cs).borrow_mut().clear();
            self.buffer.borrow(cs).borrow().clone()
        });

        assert_eq!(res.as_str(), "");
    }
}

impl TraceHandler for TestTraceHandler {
    fn trace_write(&self, msg: &dyn FixedStringRef) {
        std::println!("Got msg {}", msg.as_str());

        // Don't panic in here
        let _ = critical(|cs| self.buffer.borrow(cs).borrow_mut().concatinate(msg));
    }
}

struct TraceTestGuard {}

impl Drop for TraceTestGuard {
    fn drop(&mut self) {
        std::println!("Dropping guard");
        trace_cleanup();
    }
}

const STRING: &str = "Hello, World!";

const STRING_NEWLINE: &str = if cfg!(feature = "no-color") {
    "Hello, World!\r\n"
} else {
    "\u{1b}[0mHello, World!\r\n"
};

const STRING_DEBUG: &str = if cfg!(feature = "no-color") {
    "DEBUG: Hello, World!\r\n"
} else {
    "\u{1b}[35mDEBUG: Hello, World!\u{1b}[0m\r\n"
};

const STRING_INFO: &str = if cfg!(feature = "no-color") {
    "INFO: Hello, World!\r\n"
} else {
    "\u{1b}[32mINFO: Hello, World!\u{1b}[0m\r\n"
};

const STRING_WARNING: &str = if cfg!(feature = "no-color") {
    "WARNING: Hello, World!\r\n"
} else {
    "\u{1b}[33mWARNING: Hello, World!\u{1b}[0m\r\n"
};

const STRING_ERROR: &str = if cfg!(feature = "no-color") {
    "ERROR: Hello, World!\r\n"
} else {
    "\u{1b}[31mERROR: Hello, World!\u{1b}[0m\r\n"
};

const STRING_PANIC: &str = if cfg!(feature = "no-color") {
    "PANIC: Hello, World!\r\n"
} else {
    "\u{1b}[31mPANIC: Hello, World!\u{1b}[0m\r\n"
};

#[test]
fn create_and_cleanup() {
    critical(|_| {
        let _ = TraceTestGuard {};
        static TRACE_HANDLER: TestTraceHandler = TestTraceHandler::new();
        TRACE_HANDLER.reset();
        trace_setup(&TRACE_HANDLER);
        trace_cleanup();
    })
}

#[test]
fn trace_string() {
    critical(|_| {
        let _ = TraceTestGuard {};

        static TRACE_HANDLER: TestTraceHandler = TestTraceHandler::new();
        TRACE_HANDLER.reset();
        trace_setup(&TRACE_HANDLER);

        trace!("{}", STRING);
        let res = TRACE_HANDLER.get_result();
        assert_eq!(STRING, res.as_str());
        trace_cleanup();
    })
}

#[test]
fn trace_newline() {
    critical(|_| {
        let _ = TraceTestGuard {};
        static TRACE_HANDLER: TestTraceHandler = TestTraceHandler::new();
        TRACE_HANDLER.reset();
        trace_setup(&TRACE_HANDLER);

        traceln!("{}", STRING);
        let res = TRACE_HANDLER.get_result();
        assert_eq!(STRING_NEWLINE, res.as_str());
        trace_cleanup();
    })
}

#[test]
fn trace_debug() {
    critical(|_| {
        let _ = TraceTestGuard {};
        static TRACE_HANDLER: TestTraceHandler = TestTraceHandler::new();
        TRACE_HANDLER.reset();
        trace_setup(&TRACE_HANDLER);

        trace_debug!("{}", STRING);
        let res = TRACE_HANDLER.get_result();
        assert_eq!(STRING_DEBUG, res.as_str());
        trace_cleanup();
    })
}

#[test]
fn trace_info() {
    critical(|_| {
        let _ = TraceTestGuard {};
        static TRACE_HANDLER: TestTraceHandler = TestTraceHandler::new();
        TRACE_HANDLER.reset();
        trace_setup(&TRACE_HANDLER);

        trace_info!("{}", STRING);
        let res = TRACE_HANDLER.get_result();
        assert_eq!(STRING_INFO, res.as_str());
        trace_cleanup();
    })
}

#[test]
fn trace_warning() {
    critical(|_| {
        let _ = TraceTestGuard {};
        static TRACE_HANDLER: TestTraceHandler = TestTraceHandler::new();
        TRACE_HANDLER.reset();
        trace_setup(&TRACE_HANDLER);

        trace_warning!("{}", STRING);
        let res = TRACE_HANDLER.get_result();
        assert_eq!(STRING_WARNING, res.as_str());
        trace_cleanup();
    })
}

#[test]
fn trace_error() {
    critical(|_| {
        let _ = TraceTestGuard {};
        static TRACE_HANDLER: TestTraceHandler = TestTraceHandler::new();
        TRACE_HANDLER.reset();
        trace_setup(&TRACE_HANDLER);

        trace_error!("{}", STRING);
        let res = TRACE_HANDLER.get_result();
        assert_eq!(STRING_ERROR, res.as_str());
        trace_cleanup();
    })
}

#[test]
fn trace_panic() {
    critical(|_| {
        let _ = TraceTestGuard {};
        static TRACE_HANDLER: TestTraceHandler = TestTraceHandler::new();
        TRACE_HANDLER.reset();
        trace_setup(&TRACE_HANDLER);

        trace_panic!("{}", STRING);
        let res = TRACE_HANDLER.get_result();
        assert_eq!(STRING_PANIC, res.as_str());
        trace_cleanup();
    })
}

#[test]
fn trace_panic_shouldnt_panic() {
    critical(|_| {
        let _ = TraceTestGuard {};
        trace_cleanup();
        trace_panic!("I should not cause a panic");
        trace_cleanup();
    })
}

#[test]
fn trace_once_only_traces_once() {
    critical(|_| {
        let _ = TraceTestGuard {};
        static TRACE_HANDLER: TestTraceHandler = TestTraceHandler::new();
        TRACE_HANDLER.reset();
        trace_setup(&TRACE_HANDLER);

        fn trace() {
            trace_once!("{}", STRING);
        }
        trace();
        trace();

        let res = TRACE_HANDLER.get_result();
        assert_eq!(STRING, res.as_str());
        trace_cleanup();
    })
}

#[test]
fn trace_ln_only_traces_once() {
    critical(|_| {
        let _ = TraceTestGuard {};

        static TRACE_HANDLER: TestTraceHandler = TestTraceHandler::new();
        TRACE_HANDLER.reset();
        trace_setup(&TRACE_HANDLER);

        fn traceln() {
            traceln_once!("{}", STRING);
        }
        traceln();
        traceln();

        let res = TRACE_HANDLER.get_result();
        assert_eq!(STRING_NEWLINE, res.as_str());
        trace_cleanup();
    })
}

#[test]
fn trace_debug_only_traces_once() {
    critical(|_| {
        let _ = TraceTestGuard {};

        static TRACE_HANDLER: TestTraceHandler = TestTraceHandler::new();
        TRACE_HANDLER.reset();
        trace_setup(&TRACE_HANDLER);

        fn trace_debug() {
            trace_debug_once!("{}", STRING);
        }
        trace_debug();
        trace_debug();

        let res = TRACE_HANDLER.get_result();
        assert_eq!(STRING_DEBUG, res.as_str());
        trace_cleanup();
    })
}

#[test]
fn trace_info_only_traces_once() {
    critical(|_| {
        let _ = TraceTestGuard {};

        static TRACE_HANDLER: TestTraceHandler = TestTraceHandler::new();
        TRACE_HANDLER.reset();
        trace_setup(&TRACE_HANDLER);

        fn trace_info() {
            trace_info_once!("{}", STRING);
        }
        trace_info();
        trace_info();

        let res = TRACE_HANDLER.get_result();
        assert_eq!(STRING_INFO, res.as_str());
        trace_cleanup();
    })
}

#[test]
fn trace_warning_once_only_traces_once() {
    critical(|_| {
        let _ = TraceTestGuard {};

        static TRACE_HANDLER: TestTraceHandler = TestTraceHandler::new();
        TRACE_HANDLER.reset();
        trace_setup(&TRACE_HANDLER);

        fn warn() {
            trace_warning_once!("{}", STRING);
        }
        warn();
        warn();

        let res = TRACE_HANDLER.get_result();
        assert_eq!(STRING_WARNING, res.as_str());
        trace_cleanup();
    })
}

#[test]
fn trace_error_only_traces_once() {
    critical(|_| {
        let _ = TraceTestGuard {};

        static TRACE_HANDLER: TestTraceHandler = TestTraceHandler::new();
        TRACE_HANDLER.reset();
        trace_setup(&TRACE_HANDLER);

        fn trace_info() {
            trace_error_once!("{}", STRING);
        }
        trace_info();
        trace_info();

        let res = TRACE_HANDLER.get_result();
        assert_eq!(STRING_ERROR, res.as_str());
        trace_cleanup();
    })
}
