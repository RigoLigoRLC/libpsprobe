
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef size_t lsprobe_status;
lsprobe_status lsprobe_families_get(void **families, size_t *count);
lsprobe_status lsprobe_families_get_name(void *family, size_t index,
                                         char **name, size_t *name_length);
lsprobe_status lsprobe_families_destroy(void *families);

int main() {
  void *families;
  lsprobe_status status = 0;

  size_t families_size = 0;
  status = lsprobe_families_get(&families, &families_size);
  printf(
      "lsprobe_families_get: return %lx; families: %p, families_size: %ld;\n",
      status, families, families_size);

  char *family_name_buffer = malloc(32);
  size_t family_name_buffer_size = 32;
  for (size_t i = 0; i < families_size; i++) {
    char *family_name;
    size_t name_length;

    status = lsprobe_families_get_name(families, i, &family_name, &name_length);
    printf("lsprobe_families_get_name: %lx; name: %p, length: %ld; ", status,
           family_name, name_length);

    if (family_name_buffer_size < name_length + 1) {
      family_name_buffer = realloc(family_name_buffer, name_length + 1);
      family_name_buffer_size = name_length + 1;
    }
    memcpy(family_name_buffer, family_name, name_length);
    family_name_buffer[name_length] = '\0';
    printf("pretty name: %s;\n", family_name_buffer);
  }

  free(family_name_buffer);

  status = lsprobe_families_destroy(families);
  printf("lsprobe_families_destroy: %lx;\n", status);

  return 0;
}
