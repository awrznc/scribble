#include <stdio.h>
#include <math.h>
#define N 1000000
int search_array[N];

// 配列の中身を表示する
int print_array() {
    for( int i=0; i<N; i++ ) {
        if(search_array[i] != 0 && search_array[i] != 1){
            printf("%d\n", search_array[i]);
        }
    }
    return 0;
}

// ロジック
int main() {

    // 配列の初期化
    for( int i=0; i<N; i++ ) search_array[i] = i;

    // N の平方根まで繰り返し篩をかける
    double square_root = sqrt(N);
    for( int i=2; i<square_root; i++ ) {
        for( int j=0; j<N; j++ ){

            // 0 の場合はcontinue
            if( search_array[j] == 0 ) continue;

            if( search_array[j] % i == 0 && search_array[j] != i ) {
                search_array[j] = 0;
            }
        }
    }

    // 配列の中身の表示
    // print_array();
    return 0;
}
