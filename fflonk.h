#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define MAX_HASH_LEN 25

bool verify_proof(const uint8_t *proof, const uint8_t *pubs);
