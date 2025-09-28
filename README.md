# Rust trace

Simple Rust trace implementation intended for `no_std` targets. Creates a
`TraceHandler` trait that can be used to setup a global trace handler. This
handler is then used by various trace macros.

**Example**

Setup a trace handler:

```rust
struct MyTraceHandler {}

impl TraceHandler for MyTraceHandler {
    fn trace_write(&self, msg: &dyn fixed_string::FixedStringRef) {
        std::print!("{}", msg);
    }
}

trace_setup(&MyTraceHandler {});
```

Then, from different parts of the codebase, the different trace macros can be used.

```rust
trace!("I am a {} trace", "normal");
traceln!("I am a {} trace", "normal newline");
trace_debug!("I am a {} trace", "debug");
trace_info!("I am a {} trace", "info");
trace_warning!("I am a {} trace", "warning");
trace_error!("I am a {} trace", "error");
trace_panic!("I am a {} trace", "panic");
```

There are also macros that can be used for tracing something only once.

```rust
trace_once!("I am a {} trace", "normal once");
traceln_once!("I am a {} trace", "normal newline once");
trace_debug_once!("I am a {} trace", "debug once");
trace_info_once!("I am a {} trace", "info once");
trace_warning_once!("I am a {} trace", "warning once");
trace_error_once!("I am a {} trace", "error once");
```
