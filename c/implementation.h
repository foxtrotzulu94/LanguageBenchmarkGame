#ifndef __IMPL
#define __IMPL

#include <stdio.h>
#include "types.h"

// Checksum function implementations
char* MD5_checksum(FILE*);
char* SHA1_checksum(FILE*);
char* SHA256_checksum(FILE*);

// TODO: find dictionary/hash map type for C
// dict scan_directory(const char * dir_name, )

#endif
