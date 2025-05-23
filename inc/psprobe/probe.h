/*
    libpsprobe C API

    probe.h: probe-rs Probe resource getter (originating from Lister)
*/

#pragma once

#include "base.h"
#include <cstdint>

#ifdef __cplusplus
extern "C" {
#endif

enum psprobe_protocol {
    proto_unspecified = 0,
    proto_swd = 1,
    proto_jtag = 2,
};

psprobe_status psprobe_probe_list_get(void **probes, size_t *count);
psprobe_status psprobe_probe_list_get_probe(void *probes, size_t index, void **probe);
psprobe_status psprobe_probe_list_destroy(void *probes);

psprobe_status psprobe_probe_get_name(void *probe, char **name, size_t *name_length);
psprobe_status psprobe_probe_get_vid_pid(void *probe, uint16_t *vid, uint16_t *pid);
psprobe_status psprobe_probe_get_serial_number(void *probe, char **serial_number, size_t *serial_number_length);
psprobe_status psprobe_probe_destroy(void *probe);

psprobe_status psprobe_probe_open(void *probe_info, void **probe);
psprobe_status psprobe_probe_set_connection_speed(void *probe, uint32_t speed);
psprobe_status psprobe_probe_get_connection_speed(void *probe, uint32_t *speed);
psprobe_status psprobe_probe_set_protocol(void *probe, uint32_t protocol);
psprobe_status psprobe_probe_get_protocol(void *probe, uint32_t *protocol);
psprobe_status psprobe_probe_close(void *probe);

#ifdef __cplusplus
}
#endif
