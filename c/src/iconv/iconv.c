#include <stdio.h>
#include <iconv.h>
#include <string.h>

int main() {

    // before chars
    char *before = "サンプルです";
    char *p_before = before;
    size_t before_size = strlen( before );

    // after chars
    char after[BUFSIZ];
    char *p_after = after;
    size_t after_size = BUFSIZ - 1;

    // convert chars
    iconv_t cd = iconv_open("UTF-8", "Shift_JIS");
    while ( 0 < before_size ) {
        iconv(cd, &p_before, &before_size, &p_after, &after_size);
    }
    iconv_close(cd);

    // print chars
    printf("before convert: %s\n", before); // ?T???v?????
    printf("after convert:  %s\n", after);  // サンプルです
    return 0;
}
