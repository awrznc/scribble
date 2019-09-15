#include <iostream>

#include "hoge.hpp"

int main(){
    // 引数なし宣言の場合
    Hoge *hoge_pointer = new Hoge;
    hoge_pointer->Display();       // num = 0;
    delete hoge_pointer;

    // 引数あり宣言の場合
    Hoge *neo_hoge_pointer = new Hoge(5);
    neo_hoge_pointer->Display();   // num = 5;
    delete hoge_pointer;

    return 0;
}
