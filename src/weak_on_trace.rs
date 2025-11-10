#[cfg(target_arch = "arm")]
use core::arch::global_asm;

// Create a weak symbol for _on_trace for ARM compilers. This should be updated to a #[weak]
// attribute when that is part of the stable release
#[cfg(target_arch = "arm")]
global_asm!(
    r#"
    .thumb
    .weak _on_trace
    .type _on_trace, %function
_on_trace:
    bx lr
"#
);

// Dummy stub for non-ARM targets (e.g., Linux)
#[cfg(all(not(target_arch = "arm"), not(test)))]
#[unsafe(no_mangle)]
pub unsafe extern "Rust" fn _on_trace(_level: u8, _msg: &str) {}
