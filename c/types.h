#ifndef __TYPES
#define __TYPES

typedef unsigned char Byte;
typedef unsigned char bool;

#define true 1
#define false 0

#include <stdio.h>
typedef char *(*checksum_function)(FILE*);

struct arg_holder{
    // Directory to check
    const char *dir_a, *dir_b;

    // Ignore unchanged files
    bool ignoreUnchanged;

    // Short Checksum name.
    // 'adler32' is the longest expected name
    char hash_name[sizeof("adler32")+1];

    // Selected checksum
    checksum_function hash_implementation;
};

#include <time.h>

struct file_result{
    const char* filepath;
    const char* hash;
    unsigned long long size;
    struct timespec last_modified;
};

// Returns true if two file results are the same. False otherwise.
bool file_result_eq(const struct file_result* a, const struct file_result* b);

// Gets the appropriate string representation of a file_result
const char* file_result_to_string(const struct file_result* result);

#include "klib/khash.h"
KHASH_MAP_INIT_STR(scan_result, struct file_result)

#endif
