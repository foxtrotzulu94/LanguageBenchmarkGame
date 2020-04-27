#include "implementation.h"

char* MD5_checksum(FILE* file){
    return NULL;
}

char* SHA1_checksum(FILE* file){
    return NULL;
}

char* SHA256_checksum(FILE* file){
    return NULL;
}

#if defined(WIN32) || defined(_WIN32) || defined(__WIN32) && !defined(__CYGWIN__)
// Windows Implementation!
void scan_directory_implementation(khash_t(scan_result) results, const char * dir_name, checksum_function hash){
    // TODO
    // Not implemented - crash now
    int *a = NULL; a[0] = 0;
}
#else
// Unix implementation
void scan_directory_implementation(khash_t(scan_result) results, const char * dir_name, checksum_function hash){
    // TODO:
}
#endif

khash_t(scan_result) scan_directory(const char * dir_name, checksum_function hash){
    khash_t(scan_result) results;
    scan_directory_implementation(results, dir_name, hash);
    return results;
}
