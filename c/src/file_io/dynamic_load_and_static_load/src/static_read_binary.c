#include "sample.h"
#include "color.h"

void static_read_object() {
    Color* color_object = (Color*)__color_dat;
    printf(
        "red: %d, green: %d, blue: %d\n",
        color_object[0].red,
        color_object[0].green,
        color_object[0].blue
    );
}

int main() {
    static_read_object();
    return 0;
}
