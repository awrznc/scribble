#include <stdio.h>
#include <stdlib.h>

// 条件
int compare(const void *a, const void *b) {
    int an = *(int*)(a);
    int bn = *(int*)(b);
    
    if (an < bn) {
        printf("%d .. %d = Smaller\n", an, bn);
        return -1;
    } else if (an > bn) {
        printf("%d .. %d = Bigger\n", an, bn);
        return +1;
    } else {
        printf("%d .. %d = Equal\n", an, bn);
        return 0;
    }
}

int main() {
    // 配列の大きさ
    int size = 5;

    // 検索対象
    int list[] = { 1, 2, 3, 4, 5 };

    // 検索する値
    int key = 2;

    /**
     * @fn bsearch
     * @brief 配列に対してバイナリサーチ（二分探索）で検索を行なう。
     * @param key 発見したい値へのポインタ
     * @param base サーチ対象の配列へのポインタ
     * @param count サーチ対象の配列の要素数
     * @param size 要素1つの大きさ
     * @param compar 要素の大小関係を比較する関数へのポインタ
     * @return 引数 key が指し示す値と一致する要素があれば、その要素へのポインタを返す。なければヌルポインタが返される。一致する要素が複数個ある場合、どの要素へのポインタが返されるかは、未規定である。
     */
    int* result = bsearch(
        &key,
        list,
        size,
        sizeof(int),
        compare
    );

    // 結果を表示
    if (NULL != result) {
        printf("result=%d\n", *result);
    } else {
        printf("result=Not found.\n");
    }

    return 0;
}

/**
 * $ gcc main.c && ./a.exe
 * 2 .. 3 = Smaller
 * 2 .. 1 = Bigger
 * 2 .. 2 = Equal
 * result=2
 */
