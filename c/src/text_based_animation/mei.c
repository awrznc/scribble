
#include <stdio.h>
#include <unistd.h>
#include <time.h>

#define STR "\e[35m RED \e[0m"

char stage[8*8] = {
    1,1,1,1,1,4,1,1,
    4,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,1,
    1,0,3,3,3,0,0,1,
    1,0,3,2,3,0,0,1,
    1,0,3,3,3,0,0,1,
    1,0,0,0,0,0,0,4,
    1,1,1,1,1,1,1,1,
};

#include "mei.h"



int app() {
    char charactor[ANSI_CHARACTOR_MAX_SIZE] = {0};
    struct timespec req = { 0, 100 * 1000000};

    int i = 0;

    // while (1){
        i++;

        for(int j=0;j<8*8;j++) {
            switch (stage[j]) {
                case 0: {
                    printf("%s %s", "\033[48;2;0;48;48m", "\e[0m");
                    break;
                }
                case 1: {
                    ANSI_CharToANSI(charactor, '#', COLOR_06, COLOR_05);
                    printf("%s", charactor);
                    ANSI_ResetString(charactor);
                    break;
                }
                case 2: {
                    ANSI_CharToANSI(charactor, '@', COLOR_02, COLOR_03 );
                    printf("%s", charactor);
                    ANSI_ResetString(charactor);
                    break;
                }
                case 3: {
                    ANSI_CharToANSI(charactor, ' ', COLOR_00, COLOR_04 );
                    printf("%s", charactor);
                    ANSI_ResetString(charactor);
                    break;
                }
                case 4: {
                    ANSI_CharToANSI(charactor, '*', COLOR_07, COLOR_05 );
                    printf("%s", charactor);
                    ANSI_ResetString(charactor);
                    break;
                }
            }
            if (stage[j] == 0) {}
            if ((j+1) % 8 == 0) {
                printf("\n");
            }
        }

        nanosleep(&req, NULL);

        // printf("\033[1A\033[1A\033[1A\033[1A\033[1A\033[1A\033[1A\033[1A\r");

        // printf("\r%d %s\n1   \033[1A", i, STR);
        fflush(stdout);
    // }

    return 0;
}

int main() {
    // app();
    app_2();
    return 0;
}