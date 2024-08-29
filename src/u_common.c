#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

#include "u_common.h"

i32 com_errno = E_SUCCESS;

noreturn void COM_DieNoFormat(const char* format) {
    switch (com_errno) {
        case E_RUST_PANIC:
            fprintf(stderr, "rust panicked:");
            break;

        default:
            break;
    }

    fprintf(stderr, format);

    putc('\n', stderr);
    exit(DIE_RETURN_CODE);
}

noreturn void COM_Die(const char* format, ...) {
    char buf[128];
    va_list args;

    va_start(args, format);
    vsnprintf(buf, sizeof(buf), format, args);
    va_end(args);

    COM_DieNoFormat(buf);
}

void COM_SetComErrno(i32 val) {
    com_errno = val;
}