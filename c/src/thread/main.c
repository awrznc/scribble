#include<stdio.h>
#include<pthread.h>


// no_mutex_countに100,000を足す
int no_mutex_count = 0;
void *add_hundred_thousand_for_no_mutex() {
    for(int i=0; i<100000; i++) {
        no_mutex_count++;
    }
}

void no_mutex() {

    // スレッドAの作成
    pthread_t pthread_A;
    pthread_create( &pthread_A, NULL, &add_hundred_thousand_for_no_mutex, NULL );

    // スレッドBの作成
    pthread_t pthread_B;
    pthread_create( &pthread_B, NULL, &add_hundred_thousand_for_no_mutex, NULL );

    // 実行
    pthread_join(pthread_A, NULL);
    pthread_join(pthread_B, NULL);

    // 直列で処理が実行されれば20000が表示される
    // 今回は並行実行なので、140000などの値が表示される場合もある
    // (「値を読み込んで」「変更して」「書き込む」処理が二つのスレッドで同時に行われるタイミングが存在するため)
    printf("count=%d\n", no_mutex_count);
}


// 排他制御用のメモリ
pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;

int use_mutex_count = 0;
void *add_hundred_thousand_for_use_mutex() {
    for(int i=0; i<100000; i++) {

        // lockからunlockまでのセクションをクリティカルセクションという
        // クリティカルセクション内にはひとつのスレッドしか入れない
        // 従って並行処理させたい処理はクリティカルセクション外に書く必要がある
        pthread_mutex_lock(&mutex);
        use_mutex_count++;
        pthread_mutex_unlock(&mutex);
    }
}

void use_mutex() {

    // 排他制御用のメモリの初期化
    pthread_mutex_init(&mutex, NULL);

    // スレッドAの作成
    pthread_t pthread_A;
    pthread_create( &pthread_A, NULL, &add_hundred_thousand_for_use_mutex, NULL );

    // スレッドBの作成
    pthread_t pthread_B;
    pthread_create( &pthread_B, NULL, &add_hundred_thousand_for_use_mutex, NULL );

    // 実行
    pthread_join(pthread_A, NULL);
    pthread_join(pthread_B, NULL);

    // 排他制御用のメモリの破棄
    pthread_mutex_destroy(&mutex);

    // 200000になる
    printf("count=%d\n", use_mutex_count);
}


// main
int main() {

    // mutexを利用する例
    use_mutex();

    // mutexを利用しない例
    no_mutex();

    return 0;
}
