#include <stdio.h>
#include <time.h>

#define DATETIME_STR_SIZE 64

class Logger {
public:
    // ちなみにダブリン平均時は時差が秒単位、アルゼンチン標準時は時差がミリ秒単位まであるらしい
    char timezone[DATETIME_STR_SIZE];
    Logger(const char* input_time_zone) {
        char* to = timezone;
        char* from = (char*)input_time_zone;
        while((*to++ = *from++) != '\0');
    }

    // 致命的なエラー（即時対応が必要）
    void Fatal(const char* message) { print_log("[FATAL]", message); }

    // エラー（営業時間内に対応）
    void Error(const char* message) { print_log("[ERROR]", message); }

    // 次のリリースまでに対応
    void Warn(const char* message) { print_log("[WARN] ", message); }

    // システムの動作状況
    void Info(const char* message) { print_log("[INFO] ", message); }

    // デバッグ情報
    void Debug(const char* message) { print_log("[DEBUG]", message); }

private:
    bool get_the_current_datetime(char* datetime) {
        time_t time_object = time(NULL);
        char tmp_datetime[DATETIME_STR_SIZE];

        // ISO 8601
        strftime(tmp_datetime, sizeof(tmp_datetime), "%Y-%m-%dT%H:%M:%S", localtime(&time_object));

        {
            char tmp_datetime_and_timezone[DATETIME_STR_SIZE];
            sprintf(tmp_datetime_and_timezone, "%s%s", tmp_datetime, timezone);
            char* to = datetime;
            char* from = tmp_datetime_and_timezone;
            while((*to++ = *from++) != '\0');
        }
        return true;
    }

    void print_log(const char* logLevel, const char* message){
        char datetime[DATETIME_STR_SIZE];
        get_the_current_datetime(datetime);
        printf("%s %s %s\n", logLevel, datetime, message);
    }
};

int main() {
    Logger logger = Logger("+09:00");
    logger.Fatal("やばい");
    logger.Error("意図しない");
    logger.Warn("気を付けて");
    logger.Info("正常");
    logger.Debug("解析");
    return 0;
}
