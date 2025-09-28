use trace::{trace, trace_error, trace_error_once, trace_info, trace_info_once, trace_once, trace_panic, trace_setup, trace_warning, trace_warning_once, traceln, traceln_once, TraceHandler};

struct MyTraceHandler {}

impl TraceHandler for MyTraceHandler {
    fn trace_write(&self, msg: &dyn fixed_string::FixedStringRef) {
        std::print!("{}", msg);
    }
}

fn main() {
    trace_setup(&MyTraceHandler {});

    trace!("I am a {} trace\n", "normal");
    traceln!("I am a {} trace", "normal newline");
    trace_info!("I am a {} trace", "info");
    trace_warning!("I am a {} trace", "warning");
    trace_error!("I am a {} trace", "error");
    trace_panic!("I am a {} trace", "panic");

    for _ in 0..2 {
        trace_once!("I am a {} trace\n", "normal once");
        traceln_once!("I am a {} trace", "normal newline once");
        trace_info_once!("I am a {} trace", "info once");
        trace_warning_once!("I am a {} trace", "warning once");
        trace_error_once!("I am a {} trace", "error once");
    }
}
