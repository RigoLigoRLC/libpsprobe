
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef size_t psprobe_status;
psprobe_status psprobe_families_get(void **families, size_t *count);
psprobe_status psprobe_families_get_name(void *family, size_t index,
                                         char **name, size_t *name_length);
psprobe_status psprobe_families_get_variant_count(void *family, size_t index,
                                                  size_t *variant_count);
psprobe_status psprobe_families_get_variant_name(void *family, size_t index,
                                                 size_t variant_index,
                                                 char **variant_name,
                                                 size_t *variant_name_length);
psprobe_status psprobe_families_destroy(void *families);

int main() {
  void *families;
  psprobe_status status = 0;

  size_t families_size = 0;
  status = psprobe_families_get(&families, &families_size);
  printf(
      "psprobe_families_get: return %lx; families: %p, families_size: %ld;\n",
      status, families, families_size);

  char *name_buffer = malloc(32);
  size_t family_name_buffer_size = 32;
  for (size_t i = 0; i < families_size; i++) {
    char *family_name;
    size_t name_length;

    status = psprobe_families_get_name(families, i, &family_name, &name_length);
    printf("psprobe_families_get_name: %lx; name: %p, length: %ld; ", status,
           family_name, name_length);

    if (family_name_buffer_size < name_length + 1) {
      name_buffer = realloc(name_buffer, name_length + 1);
      family_name_buffer_size = name_length + 1;
    }
    memcpy(name_buffer, family_name, name_length);
    name_buffer[name_length] = '\0';
    printf("pretty name: %s;\n", name_buffer);

    // Get variants names
    size_t variant_count;
    status = psprobe_families_get_variant_count(families, i, &variant_count);
    printf("    psprobe_families_get_variant_count: %lx; count: %ld;\n", status,
           variant_count);
    for (size_t j = 0; j < variant_count; j++) {
      char *variant_name;
      size_t variant_name_length;
      status = psprobe_families_get_variant_name(families, i, j, &variant_name,
                                                 &variant_name_length);
      printf(
          "    psprobe_families_get_variant_name: %lx; name: %p, length: %ld; ",
          status, variant_name, variant_name_length);
      if (family_name_buffer_size < variant_name_length + 1) {
        name_buffer = realloc(name_buffer, variant_name_length + 1);
        family_name_buffer_size = name_length + 1;
      }
      memcpy(name_buffer, variant_name, variant_name_length);
      name_buffer[variant_name_length] = '\0';
      printf("pretty name: %s;\n", name_buffer);
    }
  }

  free(name_buffer);

  status = psprobe_families_destroy(families);
  printf("psprobe_families_destroy: %lx;\n", status);

  return 0;
}
