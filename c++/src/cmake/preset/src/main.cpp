#include <stdio.h>

int main() {
#ifdef PIYO_WIN32
    printf("Win32");
#elif PIYO_X64
    printf("x64");
#else
    printf("Unknown platform");
#endif
    return 0;
}
