// build:
//      gcc -shared test.c -o libtest.so
//
//      gcc -c color.c
//      ar r libcolor.a color.o
//
// example:
//      int main() {
//          color_t color = new();
//          set_code(&color, "ABCDEF");
//      }

// int return_argv(int argv) {
//     return argv;
// }

// typedef struct {
//     char code[2];
// } alpha_t;

// typedef struct {
//     char code[6];
//     alpha_t* alpha;
// } color_t;

// color_t* new() {
//     alpha_t alpha = { "" };
//     color_t color = { "", &alpha };
//     return &color;
// }
// #include<stdio.h>
// int set_code(color_t* color, const char *code) {
//     char* tmp_s = color->code;
//     while((*tmp_s++ = *code++) != '\0')
//         printf("str: %s, input: %d\n", code, *code);
//     return 0;
// }

// int ppp(const char* c ) {
//     printf("result: %s\n", c);
//     char* sss = "aaaaa";
//     printf("sss: %s\n", sss);
//     return 0;
// }

// int main() {
//     // color_t color = new();
//     // set_code(&color, "ABCDEF");
// }

#include <stdio.h>
#include <stdlib.h>

// 構造体
struct Database {
    int num;
    char name[20];
};

// Database型(Databaseと同じ）
// PrivateDatabase型(Database*)
typedef struct Database Database, *PrivateDatabase;

typedef void *MemPointer;

typedef void *PublicDatabase;

PublicDatabase database_init() {
    PrivateDatabase pointer;
    pointer = (PrivateDatabase)(MemPointer)calloc(sizeof(PrivateDatabase), 1);
    return (PublicDatabase)pointer;
}

int set_num(PublicDatabase data, int num) {
    PrivateDatabase p_data;
    p_data = (PrivateDatabase)data;
    p_data->num = num;
    return 0;
}

int set_name(PublicDatabase data, const char* name) {
    PrivateDatabase p_data;
    p_data = (PrivateDatabase)data;
    char* tmp_s = p_data->name;
    while((*tmp_s++ = *name++) != '\0');
    return 0;
}

int print_data(PublicDatabase data) {
    PrivateDatabase p_data;
    p_data = (PrivateDatabase)data;
    printf("data->num: %d\n", p_data->num);
    printf("data->name: %s\n", p_data->name);
    return 0;
}

int get_num(PublicDatabase data) {
    PrivateDatabase p_data;
    p_data = (PrivateDatabase)data;
    return p_data->num;
}

int get_name(PublicDatabase data, char* name) {
    PrivateDatabase p_data;
    p_data = (PrivateDatabase)data;
    char* tmp_s = p_data->name;
    while((*name++ = *tmp_s++) != '\0');
    return 0;
}

int database_delete(PublicDatabase data) {
    free(data);
    return 0;
}

// int main() {
//     PublicDatabase database_pointer;
//     database_pointer = database_init();
//     set_num(database_pointer, 100);
//     set_name(database_pointer, "hello");
//     print_data(database_pointer);
//     char* name;
//     get_name(database_pointer, name);
//     database_delete(database_pointer);

//     printf("name: %s", name);

//     return 0;
// }
