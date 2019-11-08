#ifndef __PROGRAM
#define __PROGRAM

#include <time.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "argument_holder.h"
#include "argtable/argtable3.h"

const char* time_format = "%F %H:%M:%S";

char* format_time();

void print_time(char* fmt_string);

#endif
