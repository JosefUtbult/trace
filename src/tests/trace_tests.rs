use core::cell::RefCell;
use critical_section::{Mutex, with as critical};

use crate::{
    TraceString, format, trace, trace_debug, trace_debug_once, trace_error, trace_error_once,
    trace_handler, trace_info, trace_info_once, trace_once, trace_panic, trace_warning,
    trace_warning_once, traceln, traceln_once,
};

static TEST_TRACE_HANDLER: TestTraceHandler = TestTraceHandler::new();

#[trace_handler]
fn on_trace(msg: &str) {
    TEST_TRACE_HANDLER.trace_write(msg);
}

type TraceBuffer = Mutex<RefCell<TraceString>>;

struct TestTraceHandler {
    buffer: TraceBuffer,
}

impl TestTraceHandler {
    const fn new() -> Self {
        Self {
            buffer: Mutex::new(RefCell::new(TraceString::new())),
        }
    }

    fn get_result(&self) -> TraceString {
        let res = critical(|cs| self.buffer.borrow(cs).borrow().clone());
        std::println!("get_result {}", res.to_string());
        res
    }

    fn reset(&self) {
        std::println!("reset");
        critical(|cs| *self.buffer.borrow(cs).borrow_mut() = TraceString::new())
    }

    fn trace_write(&self, msg: &str) {
        std::println!("Got msg {}", msg);

        let _ = critical(|cs| {
            let mut current_ref = self.buffer.borrow(cs).borrow_mut();
            *current_ref = format(format_args!("{}{}", current_ref.to_string(), msg));
        });
    }
}

struct TraceTestGuard {}
impl Drop for TraceTestGuard {
    fn drop(&mut self) {
        std::println!("Dropping guard");
        TEST_TRACE_HANDLER.reset();
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
fn trace_string() {
    critical(|_| {
        let _ = TraceTestGuard {};
        TEST_TRACE_HANDLER.reset();

        trace!("{}", STRING);
        let res = TEST_TRACE_HANDLER.get_result();
        assert_eq!(STRING, res.to_string());
    })
}

#[test]
fn trace_newline() {
    critical(|_| {
        // let _ = TraceTestGuard {};
        TEST_TRACE_HANDLER.reset();

        traceln!("{}", STRING);
        let res = TEST_TRACE_HANDLER.get_result();
        assert_eq!(STRING_NEWLINE, res.to_string());
    })
}

#[test]
fn trace_debug() {
    critical(|_| {
        let _ = TraceTestGuard {};
        TEST_TRACE_HANDLER.reset();

        trace_debug!("{}", STRING);
        let res = TEST_TRACE_HANDLER.get_result();
        assert_eq!(STRING_DEBUG, res.to_string());
    })
}

#[test]
fn trace_info() {
    critical(|_| {
        let _ = TraceTestGuard {};
        TEST_TRACE_HANDLER.reset();

        trace_info!("{}", STRING);
        let res = TEST_TRACE_HANDLER.get_result();
        assert_eq!(STRING_INFO, res.to_string());
    })
}

#[test]
fn trace_warning() {
    critical(|_| {
        let _ = TraceTestGuard {};
        TEST_TRACE_HANDLER.reset();

        trace_warning!("{}", STRING);
        let res = TEST_TRACE_HANDLER.get_result();
        assert_eq!(STRING_WARNING, res.to_string());
    })
}

#[test]
fn trace_error() {
    critical(|_| {
        let _ = TraceTestGuard {};
        TEST_TRACE_HANDLER.reset();

        trace_error!("{}", STRING);
        let res = TEST_TRACE_HANDLER.get_result();
        assert_eq!(STRING_ERROR, res.to_string());
    })
}

#[test]
fn trace_panic() {
    critical(|_| {
        let _ = TraceTestGuard {};
        TEST_TRACE_HANDLER.reset();

        trace_panic!("{}", STRING);
        let res = TEST_TRACE_HANDLER.get_result();
        assert_eq!(STRING_PANIC, res.to_string());
    })
}

#[test]
fn trace_once_only_traces_once() {
    critical(|_| {
        let _ = TraceTestGuard {};
        TEST_TRACE_HANDLER.reset();

        fn trace() {
            trace_once!("{}", STRING);
        }
        trace();
        trace();

        let res = TEST_TRACE_HANDLER.get_result();
        assert_eq!(STRING, res.to_string());
    })
}

#[test]
fn trace_ln_only_traces_once() {
    critical(|_| {
        let _ = TraceTestGuard {};
        TEST_TRACE_HANDLER.reset();

        fn traceln() {
            traceln_once!("{}", STRING);
        }
        traceln();
        traceln();

        let res = TEST_TRACE_HANDLER.get_result();
        assert_eq!(STRING_NEWLINE, res.to_string());
    })
}

#[test]
fn trace_debug_only_traces_once() {
    critical(|_| {
        let _ = TraceTestGuard {};
        TEST_TRACE_HANDLER.reset();

        fn trace_debug() {
            trace_debug_once!("{}", STRING);
        }
        trace_debug();
        trace_debug();

        let res = TEST_TRACE_HANDLER.get_result();
        assert_eq!(STRING_DEBUG, res.to_string());
    })
}

#[test]
fn trace_info_only_traces_once() {
    critical(|_| {
        let _ = TraceTestGuard {};
        TEST_TRACE_HANDLER.reset();

        fn trace_info() {
            trace_info_once!("{}", STRING);
        }
        trace_info();
        trace_info();

        let res = TEST_TRACE_HANDLER.get_result();
        assert_eq!(STRING_INFO, res.to_string());
    })
}

#[test]
fn trace_warning_once_only_traces_once() {
    critical(|_| {
        let _ = TraceTestGuard {};
        TEST_TRACE_HANDLER.reset();

        fn warn() {
            trace_warning_once!("{}", STRING);
        }
        warn();
        warn();

        let res = TEST_TRACE_HANDLER.get_result();
        assert_eq!(STRING_WARNING, res.to_string());
    })
}

#[test]
fn trace_error_only_traces_once() {
    critical(|_| {
        let _ = TraceTestGuard {};
        TEST_TRACE_HANDLER.reset();

        fn trace_info() {
            trace_error_once!("{}", STRING);
        }
        trace_info();
        trace_info();

        let res = TEST_TRACE_HANDLER.get_result();
        assert_eq!(STRING_ERROR, res.to_string());
    })
}
