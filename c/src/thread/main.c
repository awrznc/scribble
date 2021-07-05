#include<stdio.h>
#include<pthread.h>

int count = 0;

// countに100,000を足す
void *add_hundred_thousand() {
    for(int i=0; i<100000; i++) {
        count++;
    }
}

// main
int main() {
    // スレッドAの作成
    pthread_t pthread_A;
    pthread_create( &pthread_A, NULL, &add_hundred_thousand, NULL );

    // スレッドBの作成
    pthread_t pthread_B;
    pthread_create( &pthread_B, NULL, &add_hundred_thousand, NULL );

    // 実行
    pthread_join(pthread_A, NULL);
    pthread_join(pthread_B, NULL);

    // 直列で処理が実行されれば20000が表示される
    // 今回は並行実行なので、140000などの値が表示される場合もある
    // (「値を読み込んで」「変更して」「書き込む」処理が二つのスレッドで同時に行われるタイミングが存在するため)
    printf("count=%d\n", count);

    return 0;
}
