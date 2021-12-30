#include <stdio.h>
#include <stdlib.h>

#define MAP_X_SIZE 90
#define MAP_Y_SIZE 30

typedef struct {
    unsigned int x;
    unsigned int y;
} Coord;

typedef struct {
    unsigned int width;
    unsigned int height;
} Size;

typedef struct {
    /**
     * 2進数で木構造を表す
     *        1
     *        A
     *      10 11
     *    A       A
     * 100 101 110 111
     **/
    unsigned char id;
    Coord coord;
    Size size;
} Section;


typedef struct {
    Section* data;
    unsigned int current_size;
    unsigned int full_size;
} SectonList;


Section* SectonList_GetSection(SectonList* list) {
    if (list->full_size == list->current_size) return NULL;
    list->current_size++;
    return &(list->data[list->current_size]);
}


int get_random_number(int lower, int upper) {
    if (lower >= upper) return lower;

    int return_value = 0;
    return_value = (rand() % (upper - lower)) + lower;
    // printf("rand_pre: %d <= %d ~ %d\n", return_value, lower, upper);
    return return_value;
}

int get_random_boolean() {
    return rand() % 2;
}

#define true 1
#define false 0
#define bool char

int split(SectonList* list, Section* section) {

    // 部屋二つ分の大きさが区間に存在しない場合は分割できないため諦める
    int min_room_width_size = 5;
    int min_room_height_size = 3;
    int min_section_width_size = (min_room_width_size * 2) + 1;
    int min_section_height_size = (min_room_height_size * 2) + 1;
    if ( section->size.width < min_section_width_size || section->size.height < min_section_height_size ) return 0;

    // 分割処理後にリストが溢れてしまう場合は諦める
    if (list->current_size + 2 > list->full_size) return 0;

    // 50%の確率で分割する
    // if (get_random_boolean()) return 0;

    // 分割後の区間の作成
    Section* new_section_a = SectonList_GetSection(list);
    new_section_a->id = section->id;
    new_section_a->id <<= 1;

    Section* new_section_b = SectonList_GetSection(list);
    new_section_b->id = section->id;
    new_section_b->id <<= 1;
    new_section_b->id += 1;

    // DEBUG: 調整
    get_random_boolean();
    get_random_boolean();
    get_random_boolean();
    get_random_boolean();
    get_random_boolean();

    // 縦に分割するか、横に分割するか
    if (get_random_boolean()) {
        int split_point_x = get_random_number(min_room_width_size + 1, section->size.width - min_room_width_size) + section->coord.x;

        new_section_a->coord.x = section->coord.x;
        new_section_a->coord.y = section->coord.y;
        new_section_a->size.width = (section->size.width - 1) - (section->size.width + section->coord.x - split_point_x);
        new_section_a->size.height = section->size.height;

        new_section_b->coord.x = split_point_x;
        new_section_b->coord.y = section->coord.y;
        new_section_b->size.width = (section->size.width + section->coord.x) - (split_point_x);
        new_section_b->size.height = section->size.height;
    } else {
        int split_point_y = get_random_number(min_room_height_size + 1, section->size.height - min_room_height_size) + section->coord.y;

        new_section_a->coord.x = section->coord.x;
        new_section_a->coord.y = section->coord.y;
        new_section_a->size.width = section->size.width;
        new_section_a->size.height = (section->size.height - 1) - (section->size.height + section->coord.y - split_point_y);

        new_section_b->coord.x = section->coord.x;
        new_section_b->coord.y = split_point_y;
        new_section_b->size.width = section->size.width;
        new_section_b->size.height = (section->size.height + section->coord.y) - split_point_y;
    }

    split(list, new_section_a);
    split(list, new_section_b);

    return 0;
}


int Map_Split(char stage[MAP_Y_SIZE*MAP_X_SIZE]) {

    int room_x_min = 7;
    int room_y_min = 5;

    int x_size = MAP_X_SIZE;
    int y_size = MAP_Y_SIZE;
    int road_size = 1;

    int loop_flag = 1;

    Section sections[100] = { 0 };
    SectonList list = { 0 };
    list.data = sections;
    list.full_size = 100;
    list.current_size = 0;

    Section* section = SectonList_GetSection(&list);
    section->id = 0b00000001;
    section->size.width = MAP_X_SIZE;
    section->size.height = MAP_Y_SIZE;
    split(&list, section);

    // for (int i = 1; i < list.current_size+1; i++) {
    //     printf(
    //         "%02d ... width:%02d, height:%02d, x:%d, y:%d\n",
    //         list.data[i].id,
    //         list.data[i].size.width,
    //         list.data[i].size.height,
    //         list.data[i].coord.x,
    //         list.data[i].coord.y
    //     );
    // }

    // for (int i = 0; i < MAP_Y_SIZE; i++) {
    //     for (int j = 0; j < MAP_X_SIZE; j++) {
    //         stage[j + i * MAP_X_SIZE] = 'a';
    //         printf("%c", stage[j + i * MAP_X_SIZE]);
    //     }
    //     printf("\n");
    // }
    for (int i = 0; i < MAP_Y_SIZE; i++) {
        for (int j = 0; j < MAP_X_SIZE; j++) {

            // xの位置がjより小さい場合は塗りつぶす
            char value = 0;
            for (int h = 1; h < list.current_size+1; h++) {
                if (
                    list.data[h].coord.x <= j &&
                    list.data[h].coord.y <= i &&
                    list.data[h].size.width + list.data[h].coord.x > j &&
                    list.data[h].size.height + list.data[h].coord.y > i
                ) value = list.data[h].id;
            }
            printf("\e[48;5;%dm%d\e[0m", value, (j%10));
        }
        printf("\n");
    }

    // 区切った区画にランダムな大きさの部屋をランダムに配置する
    // 部屋から区画に対して通路を作成する

    return 0;
}


int main() {
    char stage[MAP_Y_SIZE*MAP_X_SIZE] = { 0 };
    for (int i = 0; i < MAP_Y_SIZE*MAP_X_SIZE; i++) {
        stage[i] = 'a';
    }

    Map_Split(stage);

    printf("finish!\n");

    return 0;
}
