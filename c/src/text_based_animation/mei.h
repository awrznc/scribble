typedef signed char i8;
typedef unsigned char u8;
typedef signed short int i16;
typedef unsigned short int u16;
typedef signed long int i32;
typedef unsigned long int u32;
typedef signed long long int i64;
typedef unsigned long long int u64;

#define MEI_SCREEN_SIZE_X (640 / 8)
#define MEI_SCREEN_SIZE_Y (480 / 16)

#define COLOR_00 0,0,0        // black
#define COLOR_01 255,255,255  // white
#define COLOR_02 255,165,0    // player
#define COLOR_03 112,88,64    // player floor
#define COLOR_04 24,32,32     // player vicinity floor
#define COLOR_05 0,64,64      // wall
#define COLOR_06 0,128,128    // plant
#define COLOR_07 127,64,127   // flower
#define COLOR_08 48,64,64      // wall extend
#define COLOR_09 0,48,48      // floor
#define COLOR_10
#define COLOR_11
#define COLOR_12
#define COLOR_13
#define COLOR_14
#define COLOR_15

static u8 meiStage[MEI_SCREEN_SIZE_X*MEI_SCREEN_SIZE_Y] = { 0 };

#include <stdio.h>
#include <time.h>

#include "ansi.h"

#define MEI_WALL '#'
#define MEI_FLOOR ' '
#define MEI_PLAYER '@'

static u8 aaa[MEI_SCREEN_SIZE_X*MEI_SCREEN_SIZE_Y] = {
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,4,4,1,4,4,1,4,1,4,4,1,4,4,4,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,4,4,4,4,1,4,4,1,4,1,4,4,1,4,4,4,1,4,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,4,4,4,4,4,1,4,4,4,4,1,4,4,1,4,4,4,1,4,4,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,4,4,4,4,4,1,4,4,4,4,1,4,4,1,4,4,4,4,4,4,4,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,4,4,4,4,4,1,4,4,4,4,1,4,4,4,4,4,4,4,4,4,4,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,4,1,4,4,4,4,4,1,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,4,1,4,4,4,4,4,1,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,4,1,4,4,4,4,4,4,4,4,4,0,0,4,4,4,4,4,4,4,4,4,4,4,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,4,4,4,4,4,4,4,4,4,4,4,0,0,4,4,4,4,4,4,4,4,4,4,4,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,4,4,4,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,4,4,4,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,4,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,4,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,10,3,10,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,2,2,2,2,2,2,2,2,10,10,10,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
};

int stage2() {
    // srand( (int)time(NULL) );
    // int a = rand() % (MEI_SCREEN_SIZE_Y + 1);
    // printf("%d\n", a);

    // char charactor[ANSI_CHARACTOR_MAX_SIZE] = {0};
    // for(int i=0; i<MEI_SCREEN_SIZE_X*MEI_SCREEN_SIZE_Y; i++) {
    //     ANSI_CharToANSI(charactor, '0', COLOR_01, COLOR_00);
    //     printf("%s", charactor);
    //     ANSI_ResetString(charactor);
    //     if ((i+1) % MEI_SCREEN_SIZE_X == 0) printf("\n");
    // }

    struct timespec req = { 0, 100 * 1000000};

    char charactor[ANSI_CHARACTOR_MAX_SIZE] = {0};

    int drop1 = 12;
    int drop2 = 48;
    int cnt = 0;


    int drop1_left = 18;
    int drop2_left = 39;
    int cnt_left = 0;

    int drop1_left_left = 16;
    int drop2_left_left = 32;
    int cnt_left_left = 0;

    while (1){

    if (drop1 < MEI_SCREEN_SIZE_Y - 15) {
        aaa[MEI_SCREEN_SIZE_X*(drop1-1) + drop2] = 4;
        aaa[MEI_SCREEN_SIZE_X*drop1 + drop2] = 5;
        drop1++;
    } else if(drop1 < MEI_SCREEN_SIZE_Y - 14) {
        aaa[MEI_SCREEN_SIZE_X*drop1 + drop2] = 6;
        aaa[MEI_SCREEN_SIZE_X*(drop1-1) + drop2] = 4;
        drop1++;
    } else if(drop1 < MEI_SCREEN_SIZE_Y - 12) {
        aaa[MEI_SCREEN_SIZE_X*drop1 + drop2] = 6;
        aaa[MEI_SCREEN_SIZE_X*(drop1-1) + drop2] = 2;
        drop1++;
    } else {
        aaa[MEI_SCREEN_SIZE_X*(drop1-1) + drop2] = 2;
        if (cnt == 0) {
            aaa[MEI_SCREEN_SIZE_X*(drop1-1) + drop2] = 7;
            cnt++;
        } else if(cnt == 1) {
            aaa[MEI_SCREEN_SIZE_X*(drop1-1) + drop2-1] = 8;
            aaa[MEI_SCREEN_SIZE_X*(drop1-1) + drop2+1] = 9;
            cnt++;
        } else {
            aaa[MEI_SCREEN_SIZE_X*(drop1-1) + drop2-1] = 2;
            aaa[MEI_SCREEN_SIZE_X*(drop1-1) + drop2+1] = 2;
            drop1 = 9;
            cnt = 0;
        }
    }

    if (drop1_left < MEI_SCREEN_SIZE_Y - 15) {
        aaa[MEI_SCREEN_SIZE_X*(drop1_left-1) + drop2_left] = 4;
        aaa[MEI_SCREEN_SIZE_X*drop1_left + drop2_left] = 5;
        drop1_left++;
    } else if(drop1_left < MEI_SCREEN_SIZE_Y - 14) {
        aaa[MEI_SCREEN_SIZE_X*drop1_left + drop2_left] = 6;
        aaa[MEI_SCREEN_SIZE_X*(drop1_left-1) + drop2_left] = 4;
        drop1_left++;
    } else if(drop1_left < MEI_SCREEN_SIZE_Y - 12) {
        aaa[MEI_SCREEN_SIZE_X*drop1_left + drop2_left] = 6;
        aaa[MEI_SCREEN_SIZE_X*(drop1_left-1) + drop2_left] = 2;
        drop1_left++;
    } else {
        aaa[MEI_SCREEN_SIZE_X*(drop1_left-1) + drop2_left] = 2;
        if (cnt_left == 0) {
            aaa[MEI_SCREEN_SIZE_X*(drop1_left-1) + drop2_left] = 7;
            cnt_left++;
        } else if(cnt_left == 1) {
            aaa[MEI_SCREEN_SIZE_X*(drop1_left-1) + drop2_left-1] = 8;
            aaa[MEI_SCREEN_SIZE_X*(drop1_left-1) + drop2_left+1] = 9;
            cnt_left++;
        } else {
            aaa[MEI_SCREEN_SIZE_X*(drop1_left-1) + drop2_left-1] = 2;
            aaa[MEI_SCREEN_SIZE_X*(drop1_left-1) + drop2_left+1] = 2;
            drop1_left = 9;
            cnt_left = 0;
        }
    }

    if (drop1_left_left < MEI_SCREEN_SIZE_Y - 15) {
        aaa[MEI_SCREEN_SIZE_X*(drop1_left_left-1) + drop2_left_left] = 4;
        aaa[MEI_SCREEN_SIZE_X*drop1_left_left + drop2_left_left] = 5;
        drop1_left_left++;
    } else if(drop1_left_left < MEI_SCREEN_SIZE_Y - 14) {
        aaa[MEI_SCREEN_SIZE_X*drop1_left_left + drop2_left_left] = 6;
        aaa[MEI_SCREEN_SIZE_X*(drop1_left_left-1) + drop2_left_left] = 4;
        drop1_left_left++;
    } else if(drop1_left_left < MEI_SCREEN_SIZE_Y - 12) {
        aaa[MEI_SCREEN_SIZE_X*drop1_left_left + drop2_left_left] = 6;
        aaa[MEI_SCREEN_SIZE_X*(drop1_left_left-1) + drop2_left_left] = 2;
        drop1_left_left++;
    } else {
        aaa[MEI_SCREEN_SIZE_X*(drop1_left_left-1) + drop2_left_left] = 2;
        if (cnt_left_left == 0) {
            aaa[MEI_SCREEN_SIZE_X*(drop1_left_left-1) + drop2_left_left] = 7;
            cnt_left_left++;
        } else if(cnt_left_left == 1) {
            aaa[MEI_SCREEN_SIZE_X*(drop1_left_left-1) + drop2_left_left-1] = 8;
            aaa[MEI_SCREEN_SIZE_X*(drop1_left_left-1) + drop2_left_left+1] = 9;
            cnt_left_left++;
        } else {
            aaa[MEI_SCREEN_SIZE_X*(drop1_left_left-1) + drop2_left_left-1] = 2;
            aaa[MEI_SCREEN_SIZE_X*(drop1_left_left-1) + drop2_left_left+1] = 2;
            drop1_left_left = 9;
            cnt_left_left = 0;
        }
    }

    for(int i=0; i<MEI_SCREEN_SIZE_X*MEI_SCREEN_SIZE_Y; i++) {
        switch (aaa[i]) {
            case 0: {
                ANSI_CharToANSI(charactor, ' ', COLOR_00, COLOR_00);
                printf("%s", charactor);
                ANSI_ResetString(charactor);
                break;
            }
            case 1: {
                ANSI_CharToANSI(charactor, '$', COLOR_06, COLOR_05);
                printf("%s", charactor);
                ANSI_ResetString(charactor);
                break;
            }
            case 2: {
                ANSI_CharToANSI(charactor, ' ', COLOR_00, COLOR_09 );
                printf("%s", charactor);
                ANSI_ResetString(charactor);
                break;
            }
            case 3: {
                ANSI_CharToANSI(charactor, '@', COLOR_02, COLOR_03 );
                printf("%s", charactor);
                ANSI_ResetString(charactor);
                break;
            }
            case 4: {
                ANSI_CharToANSI(charactor, '#', COLOR_05, COLOR_08 );
                printf("%s", charactor);
                ANSI_ResetString(charactor);
                break;
            }
            case 5: {
                ANSI_CharToANSI(charactor, '.', COLOR_01, COLOR_05 );
                printf("%s", charactor);
                ANSI_ResetString(charactor);
                break;
            }
            case 6: {
                ANSI_CharToANSI(charactor, '.', COLOR_01, COLOR_09 );
                printf("%s", charactor);
                ANSI_ResetString(charactor);
                break;
            }
            case 7: {
                ANSI_CharToANSI(charactor, 'v', COLOR_01, COLOR_09 );
                printf("%s", charactor);
                ANSI_ResetString(charactor);
                break;
            }
            case 8: {
                ANSI_CharToANSI(charactor, '(', COLOR_01, COLOR_09 );
                printf("%s", charactor);
                ANSI_ResetString(charactor);
                break;
            }
            case 9: {
                ANSI_CharToANSI(charactor, ')', COLOR_01, COLOR_09 );
                printf("%s", charactor);
                ANSI_ResetString(charactor);
                break;
            }
            case 10: {
                ANSI_CharToANSI(charactor, ' ', COLOR_00, COLOR_04 );
                printf("%s", charactor);
                ANSI_ResetString(charactor);
                break;
            }
        }
        if ((i+1) % MEI_SCREEN_SIZE_X == 0) printf("\n");
    }

    nanosleep(&req, NULL);
    for (int j = 0; j < MEI_SCREEN_SIZE_Y; j++) {
        printf("\033[1A");
    }
    fflush(stdout);

    }

    return 0;
}


int app_2() {
    char charactor[ANSI_CHARACTOR_MAX_SIZE] = {0};

    // for(int i=0; i<MEI_SCREEN_SIZE_X*MEI_SCREEN_SIZE_Y; i++) {
    //     ANSI_CharToANSI(charactor, ' ', COLOR_00, COLOR_00);
    //     printf("%s", charactor);
    //     ANSI_ResetString(charactor);
    //     if ((i+1) % MEI_SCREEN_SIZE_X == 0) printf("\n");
    // }

    stage2();
    return 0;
}

