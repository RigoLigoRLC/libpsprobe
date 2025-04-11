/*
    libpsprobe C API

    session.h: probe-rs Connection session API
*/

#pragma once

#include "base.h"
#include "bridge.h"

#ifdef __cplusplus
extern "C" {
#endif


psprobe_status psprobe_session_open(void *probe, const char *device_name, size_t device_name_len, void **session);
psprobe_status psprobe_session_close(void *session);

psprobe_status psprobe_core_info_get(void *session, void **core_info, size_t *count);
psprobe_status psprobe_core_info_get_entry(void *core_info, size_t index, size_t *core_index,
                                           psprobe_ext_dyn_str *core_desc);
psprobe_status psprobe_core_info_destroy(void *core_info);

psprobe_status psprobe_session_read_memory_8(void *session, size_t core, uint64_t address, size_t count, void *dest);
psprobe_status psprobe_session_read_memory_16(void *session, size_t core, uint64_t address, size_t count, void *dest);
psprobe_status psprobe_session_read_memory_32(void *session, size_t core, uint64_t address, size_t count, void *dest);
psprobe_status psprobe_session_read_memory_64(void *session, size_t core, uint64_t address, size_t count, void *dest);

#ifdef __cplusplus
}
#endif