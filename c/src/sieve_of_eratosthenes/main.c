#include <stdio.h>
#include <math.h>
#define N 1000000000
#define TRUE 1
#define FALSE 0
int search_array[N];

// 配列の中身を表示する
int print_array() {
    for( int i=0; i<N; i++ ) {
        if(search_array[i] == TRUE){
            printf("%d\n", i);
        }
    }
    return 0;
}

// ロジック
int main() {

    // 配列の初期化
    for( int i=0; i<N; i++ ) search_array[i] = TRUE;
    search_array[0] = FALSE;
    search_array[1] = FALSE;

    // N の平方根まで繰り返し篩をかける
    double square_root = sqrt(N);
    for( int i=2; i<square_root; i++ ) {

        // 0 の場合はcontinue
        if( search_array[i] == FALSE ) continue;

        for( int j=i*i; j<N; j+=i ){
            search_array[j] = FALSE;
        }
    }

    // 配列の中身の表示
    // print_array();
    return 0;
}
