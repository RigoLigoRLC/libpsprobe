/*
    libpsprobe C API

    bridge.h: probe-rs Miscellaneous data structure bridging utilities
*/

#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

struct psprobe_ext_dyn_str {
    const char *str;
    size_t len;
    void *rust_box;
};

void psprobe_extdynstr_dispose(struct psprobe_ext_dyn_str *str);

#ifdef __cplusplus
}
#endif
