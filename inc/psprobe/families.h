
/*
    libpsprobe C API

    families.h: probe-rs chip families resources getter
*/

#pragma once

#include "base.h"

#ifdef __cplusplus
extern "C" {
#endif

psprobe_status psprobe_families_get(void **families, size_t *count);
psprobe_status psprobe_families_get_name(void *family, size_t index, char **name, size_t *name_length);
psprobe_status psprobe_families_get_variant_count(void *family, size_t index, size_t *variant_count);
psprobe_status psprobe_families_get_variant_name(void *family, size_t index, size_t variant_index, char **variant_name,
                                                 size_t *variant_name_length);
psprobe_status psprobe_families_destroy(void *families);

#ifdef __cplusplus
}
#endif
