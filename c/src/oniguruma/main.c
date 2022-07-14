/**
  * onig_new で初期化を行い
  * onig_search で解析を行う
  **/

#include <oniguruma.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int regex(char* target_string, char* regex_pattern) {

    // 正規表現の設定を行う
    regex_t *regex_object = { 0 };
    {
        OnigErrorInfo err_info = { 0 };
        char *end = regex_pattern + strlen( regex_pattern );
        int status = onig_new(
            &regex_object,          // 作成された正規表現オブジェクトを返すアドレス
            regex_pattern,          // 正規表現パターン文字列
            end,                    // 正規表現パターン文字列の終端アドレス
            ONIG_OPTION_DEFAULT,    // 正規表現コンパイル時オプション
            ONIG_ENCODING_UTF8,     // 文字エンコーディング設定
            ONIG_SYNTAX_DEFAULT,    // 正規表現パターン文法定義
            &err_info               // エラー情報が返るアドレス
        );

        if(status != ONIG_NORMAL){
            char error_string[ONIG_MAX_ERROR_MESSAGE_LEN] = { 0 };
            onig_error_code_to_str(error_string, status, err_info);
            printf("error: %s\n", error_string);
            return 1;
        }
    }

    // 解析の実行
    char *end = target_string + strlen(target_string);
    char *range = end;
    char *start = target_string;
    OnigRegion *region = onig_region_new();
    int result = onig_search(
        regex_object,       // 正規表現オブジェクト
        target_string,      // 検索対象文字列
        end,                // 検索対象文字列の終端アドレス
        start,              // 検索対象文字列の検索先頭位置アドレス ( この仕組みを利用することで検索を途中で開始することができる )
        range,              // 検索対象文字列の検索終了位置アドレス ( この仕組みを利用することで検索を途中で終了することができる )
        region,             // マッチ領域の情報
        ONIG_OPTION_NONE    // 検索オプション
    );

    if (result < 0) {
        printf("error: match failed\n");
        return 1;
    }

    // 最初にマッチした結果の取得
    // region->num_regs の数だけマッチしている
    // 今回は決め打ちで0を指定（最初にマッチしたもののみ取得）
    size_t match_string_length = region->end[0] - region->beg[0];
    size_t match_string_size = match_string_length + 1;
    char* match_string = (char*)malloc(match_string_size);
    if (!match_string) {
        printf("error: malloc failed ( malloc(%d) )\n", match_string_size);
        return 1;
    }
    for (int i = 0; i < match_string_length; i++) {
        match_string[i] = target_string[region->beg[0] + i];
    }
    match_string[match_string_length] = '\0';

    // 最初にマッチした結果の表示
    printf("target string: %s\n", target_string);
    printf("regex pattern: %s\n", regex_pattern);
    printf("match count: %d\n", region->num_regs);
    printf("match point: %d\n", result);
    printf("match string: %s\n", match_string);

    free(match_string);
    onig_region_free(region, 1);

    return 0;
}


int main() {

    // シンプルな文字マッチ
    regex("piyo", "p");
    // =>
    // target string: piyo
    // regex pattern: p
    // match point: 0
    // match string: p
    printf("========\n");

    // メタ文字を利用した文字列マッチ
    regex("piyo", "^pi..$");
    // =>
    // target string: piyo
    // regex pattern: ^pi..$
    // match point: 0
    // match string: piyo
    printf("========\n");

    // 再帰的なマッチ
    // 対応するaとbを判定する（括弧で囲うイメージ）
    // cが挟まれていてもOK
    char* regex_pattern = "\\A(?<re>a\\g<re>*b|c)*\\z";
    regex( "ab", regex_pattern );       /* match        */  printf("========\n");
    regex( "aab", regex_pattern );      /* match failed */  printf("========\n");
    regex( "aabb", regex_pattern );     /* match        */  printf("========\n");
    regex( "aacbb", regex_pattern );    /* match        */  printf("========\n");
    regex( "aacbacbb", regex_pattern ); /* match        */  printf("========\n");

    return 0;
}
