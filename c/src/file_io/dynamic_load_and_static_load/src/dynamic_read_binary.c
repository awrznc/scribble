#include "sample.h"

void dynamic_read_object() {
    Color color_object[COLOR_ARRAY_SIZE];
    FILE *fpr = fopen("color.dat", "rb");
    fread(&color_object, sizeof(color_object), OBJ_SIZE, fpr);
    fclose(fpr);
    printf(
        "red: %d, green: %d, blue: %d\n",
        color_object[1].red,
        color_object[1].green,
        color_object[1].blue
    );
}

int main() {
    dynamic_read_object();
    return 0;
}
