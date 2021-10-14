#include "sample.h"

void write_object() {
    Color color_object[COLOR_ARRAY_SIZE] = { 0 };
    FILE *fpw = fopen("color.dat", "wb");
    fwrite(&color_object, sizeof(color_object), OBJ_SIZE, fpw);
    fclose(fpw);
    printf(
        "red: %d, green: %d, blue: %d\n",
        color_object[0].red,
        color_object[0].green,
        color_object[0].blue
    );
}

int main() {
    write_object();
    return 0;
}
