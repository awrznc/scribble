#define ANSI_PREFIX "\033["
#define ANSI_SUFFIX "\e[0m"

#define ANSI_BACKGROUND_COLOR_CODE 48
#define ANSI_FOREGROUND_COLOR_CODE 38
#define ANSI_24BIT_COLOR_CODE 2

#define ANSI_CHARACTOR_MAX_SIZE 64

#include <stdio.h>
#include <stdlib.h>


void ANSI_ResetString(char ansiString[ANSI_CHARACTOR_MAX_SIZE]) {
    for (unsigned char i = 0; i < ANSI_CHARACTOR_MAX_SIZE; i++) ansiString[i] = 0;
}

void ANSI_CharToANSI(
    char* string,
    char charactor,
    unsigned char foregroundRed, unsigned char foregroundGreen, unsigned char foregroundBlue,
    unsigned char backgroundRed, unsigned char backgroundGreen, unsigned char backgroundBlue
) {
    sprintf(
        string,
        "%s%d;%d;%d;%d;%d;%dm%s%d;%d;%d;%d;%d;%dm%c%s",

        ANSI_PREFIX,
        ANSI_FOREGROUND_COLOR_CODE,
        ANSI_24BIT_COLOR_CODE,
        foregroundRed,
        foregroundGreen,
        foregroundBlue,

        ANSI_FOREGROUND_COLOR_CODE,

        ANSI_PREFIX,
        ANSI_BACKGROUND_COLOR_CODE,
        ANSI_24BIT_COLOR_CODE,
        backgroundRed,
        backgroundGreen,
        backgroundBlue,
        ANSI_BACKGROUND_COLOR_CODE,

        charactor,
        ANSI_SUFFIX
    );
}
