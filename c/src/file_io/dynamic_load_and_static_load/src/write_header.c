#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv) {
    if(argc != 2) return 1;

    FILE *binary_file = fopen(argv[1], "rb");
    if(binary_file == NULL) return 2;

    char* file_path = argv[1];
    char* variable_name = "";
    variable_name = file_path;

    // remove `./`
    if (*variable_name == '.') {
        variable_name++;
        if (*variable_name == '/') {
            variable_name++;
        } else {
            variable_name = file_path;
        }
    }

    {
        char* temporary_search = variable_name;
        unsigned int last_slash_point = 0;
        unsigned int current_point = 0;
        while (*temporary_search != '\0') {
            current_point++;

            // replace `.` to `_`
            if (*temporary_search == '.') *temporary_search = '_';
            if (*temporary_search == '/') last_slash_point = current_point;
            temporary_search++;
        }

        // remove `/`
        variable_name += last_slash_point;
    }

    {
        int binary_data = 0;
        unsigned char count = 1;
        unsigned int size = 0;

        printf("unsigned char __%s[] = {\n    ", variable_name);
        while((binary_data = getc(binary_file)) != EOF){
            size++;
            printf("0x%02x, ", binary_data);
            if (count % 16 == 0) {
                count = 1;
                printf("\n    ");
            }
            count++;
        }
        printf("\n};\n");
        printf("unsigned int __%s_size = %u;\n", variable_name, size);
    }

    fclose(binary_file);
    return 0;
}
