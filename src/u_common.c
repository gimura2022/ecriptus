#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

#include "u_common.h"

noreturn void COM_Die(const char* format, ...) {
    va_list args;

    va_start(args, format);
    vfprintf(stderr, format, args);
    va_end(args);

    putc('\n', stderr);

    exit(DIE_RETURN_CODE);
}