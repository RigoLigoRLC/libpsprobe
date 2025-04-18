/*
    libpsprobe C API

    session.h: probe-rs Connection session API
*/

#pragma once

#include "base.h"
#include "bridge.h"
#include <stdint.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif


psprobe_status psprobe_session_open(void *probe, const char *device_name, size_t device_name_len, void **session);
psprobe_status psprobe_session_close(void *session);

psprobe_status psprobe_core_info_get(void *session, void **core_info, size_t *count);
psprobe_status psprobe_core_info_get_entry(void *core_info, size_t index, size_t *core_index,
                                           psprobe_ext_dyn_str *core_desc);
psprobe_status psprobe_core_info_destroy(void *core_info);

#ifdef __cplusplus
}
#endif