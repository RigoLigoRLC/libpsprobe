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

psprobe_status psprobe_probe_list_get(void **probes, size_t *count);
psprobe_status psprobe_probe_list_get_probe(void *probes, size_t *index, void **probe);
psprobe_status psprobe_probe_list_destroy(void *probes);

psprobe_status psprobe_probe_get_name(void *probe, char **name, size_t *name_length);
psprobe_status psprobe_probe_get_vid_pid(void *probe, uint16_t *vid, uint16_t *pid);
psprobe_status psprobe_probe_get_serial_number(void *probe, char **serial_number, size_t *serial_number_length);
psprobe_status psprobe_probe_destroy(void *probe);

#ifdef __cplusplus
}
#endif
