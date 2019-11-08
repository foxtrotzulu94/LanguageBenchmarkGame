#ifndef __ARGS
#define __ARGS

#include "types.h"

struct arg_holder{
    // Directory to check
    const char *dir_a, *dir_b;

    // Ignore unchanged files
    bool ignoreUnchanged;

    // Short Checksum name.
    // 'adler32' is the longest expected name
    char checksum[sizeof("adler32")+1];
};

#endif
