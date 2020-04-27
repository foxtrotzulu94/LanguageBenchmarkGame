#ifndef __IMPL
#define __IMPL

#include <stdio.h>
#include "types.h"

static const int BUFFER_SIZE = 64*1024; // 64KB

// Checksum function implementations
char* MD5_checksum(FILE*);
char* SHA1_checksum(FILE*);
char* SHA256_checksum(FILE*);

khash_t(scan_result) scan_directory(const char * dir_name, checksum_function hash);

#endif
