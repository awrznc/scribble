#include <stdio.h>
#include <time.h>


/*
 * 現在時刻を表示します
 */
void get_now_date_sample() {
    time_t tm = time(NULL);
    printf("now date: %s\n", ctime(&tm));
}


/*
 * 決められた日付から5日進めた日付を表示します
 */
void add_date_sample() {

    // tm 構造体の初期化
    int sec = 0;
    int min = 0;
    int hour = 0;
    int day = 6;
    int mon = 3;
    int year = 2020;
    struct tm tm = { sec, min, hour, day, mon-1, year-1900 };

    // 5日進める
    int add_day = 5;
    time_t time = mktime(&tm) + 86400 * add_day;

    // 表示用の文章の作成
    struct tm *ltime = localtime(&time);
    char after_date[11];
    sprintf(
        after_date,
        "%d-%02d-%02d",
        ltime->tm_year+1900,
        ltime->tm_mon+1,
        ltime->tm_mday
    );
    char before_date[11];
    sprintf(
        before_date,
        "%d-%02d-%02d",
        year,
        mon,
        day
    );
    printf("%sから%d日後は%sです\n", before_date, add_day, after_date);
    // => 2020-03-06から5日後は2020-03-11です
}


/*
 * main
 */
int main() {
    get_now_date_sample();
    add_date_sample();
    return 0;
}
