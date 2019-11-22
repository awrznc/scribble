#include "stdio.h"
#include "string.h"

int print_split_utf_8_str_result(char *string, char *splitter)
{
    // 区切る文字列
    char *str_p = string; // 進める用
    char *split_point = string; // 文字列コピー用
    
    // 区切り文字列
    char *spl_p = splitter;
    int spl_size = strlen(splitter);
    
    // マッチしたカウント
    int match_cnt = 0;
    
    // マッチした場所
    char *match_p = NULL;
    
    // アドレスの末尾がNULLなら終了
    while (*str_p != '\0'){
        
        // DEBUG: アドレスを16進数で表示
        // printf("%x ", *str_p);

        // 区切る文字のアドレスと区切り文字のアドレスが一致した場合はポインタを進める
        if(*str_p == *spl_p){
            spl_p++;
            match_cnt++;
        
        // それ以外はリセット
        }else{
            spl_p -= match_cnt;
            match_cnt = 0;
        }

        // 区切り文字のサイズとマッチ数のサイズが同じ場合は区切る
        if(match_cnt == spl_size){
            
            // 区切り文字のサイズだけポインタを戻しながらNULL文字で埋める
            for(int i = 0; i < spl_size ; i++){
                *str_p = '\0';
                str_p--;
            }
            
            // 埋めたらポインタを埋める前の位置に戻す
            str_p += spl_size;
            spl_p -= spl_size;
            
            // 表示
            printf("split: %s\n", split_point);
            match_cnt = 0;
            str_p++;
            split_point = str_p;
        
        // それ以外はポインタを進める
        } else {
            str_p++;
        }
    }
    
    // 残りを吐き出す
    printf("split: %s\n", split_point);
    
    return 0;
}

int main()
{
    // サイズを指定しないと、アドレスに格納されている文字をいじる際に BusError になってしまう
    char str[BUFSIZ] = "とうきょうとっきょきょかきょく";
    char *spl = "きょ";

    print_split_utf_8_str_result(str, spl);
    // =>
    // split: とう
    // split: うとっ
    // split:
    // split: か
    // split: く
    return 0;
}