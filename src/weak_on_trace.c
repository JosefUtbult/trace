#include <stddef.h>

// Use GCC/Clang weak attribute
__attribute__((weak)) void _on_trace(unsigned int, const unsigned char*, size_t) {}
