#include <stddef.h>

// Use GCC/Clang weak attribute to circumvent not having weak rust linking in
// stable. This should be updated when that is a stable feature, as this is a
// hack
__attribute__((weak)) void _on_trace(unsigned int, const unsigned char*, size_t) {}
