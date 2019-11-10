#include "types.h"
#include "program.h"

bool file_result_eq(const struct file_result* a, const struct file_result* b){
    return strcmp(a->hash, b->hash) == 0
        && strcmp(a->filepath, b->filepath) == 0
        && a->size == b->size
        // We can only use second precision rather than nanosecond
        // If only the nanoseconds are different, but the hash and the size are not
        // then we're pretty sure these are the same file
        && a->last_modified.tv_sec == b->last_modified.tv_sec;
}

// Gets the appropriate string representation of a file_result
const char* file_result_to_string(const struct file_result* result){
    // Find an appropriate buffer size
    int buf_size = strlen(result->filepath) + sizeof("1989-11-09 21:10:26") + (sizeof("18446744073709551616")*2);
    char* buffer = calloc(buf_size, sizeof(char*));

    snprintf(buffer, buf_size, "%s (%s | %llu bytes)", result->filepath, format_timespec(result->last_modified.tv_sec), result->size);

    return buffer;
}
