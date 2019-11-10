#ifndef __PROGRAM
#define __PROGRAM

#include <time.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "types.h"
#include "implementation.h"
#include "argtable/argtable3.h"

static const char* time_format = "%F %H:%M:%S";

char* format_timespec(time_t time);
char* format_time_now();

void print_time_now(char* fmt_string);

#endif
