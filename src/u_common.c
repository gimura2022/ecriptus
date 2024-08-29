#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

#include "u_common.h"

i32 com_errno = E_SUCCESS;

noreturn void COM_Die(const char* format, ...) {
    if (com_errno & 0b1) {
        switch (com_errno) {
            case E_RUST_PANIC:
                fprintf(stderr, "rust panicked:");
                break;

            default:
                fprintf(stderr, "error with code %i:", com_errno);
                break;
        }
    }

    va_list args;
    va_start(args, format);
    vfprintf(stderr, format, args);
    va_end(args);

    putc('\n', stderr);

    exit(DIE_RETURN_CODE);
}