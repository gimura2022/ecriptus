#include <stdio.h>

#include "u_common.h"

extern u32 add(u32 lhs, u32 rhs);

int main(int argc, char* argv[]) {
    printf("10 + 10 = %i\n", add(10, 10));

    return 0;
}