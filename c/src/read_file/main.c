#include <stdio.h>

int main() {
    FILE *file_pointer;
    char file_name[] = "test.txt";

    file_pointer = fopen(file_name, "r");

    if(file_pointer == NULL){
        printf("%s file not open!\n", file_name);
		return -1;
    } else {
        printf("%s file opened!\n", file_name);
    }

    fclose(file_pointer);
}
