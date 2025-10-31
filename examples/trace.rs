use trace::{
    trace_format, trace, trace_debug, trace_debug_once, trace_error, trace_error_once,
    trace_info, trace_info_once, trace_once, trace_panic, trace_warning, trace_warning_once,
    on_trace, traceln, traceln_once,
};

on_trace!(|msg| {
    std::print!("{}", msg);
});

fn main() {
    // Direct access to trace function. Allows for any size of fixed strings
    trace_format(format_args!(
        "Raw access to the underlying trace function\n"
    ));

    // Macro formatting
    trace!("I am a {} trace\n", "normal");
    traceln!("I am a {} trace", "normal newline");
    trace_debug!("I am a {} trace", "debug");
    trace_info!("I am a {} trace", "info");
    trace_warning!("I am a {} trace", "warning");
    trace_error!("I am a {} trace", "error");
    trace_panic!("I am a {} trace", "panic");

    // One time tracing
    for _ in 0..2 {
        trace_once!("I am a {} trace\n", "normal once");
        traceln_once!("I am a {} trace", "normal newline once");
        trace_debug_once!("I am a {} trace", "debug once");
        trace_info_once!("I am a {} trace", "info once");
        trace_warning_once!("I am a {} trace", "warning once");
        trace_error_once!("I am a {} trace", "error once");
    }
}
