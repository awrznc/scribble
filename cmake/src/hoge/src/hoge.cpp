#include <iostream>

#include "../include/hoge.hpp"

/**
 *  @fn     Hoge
 *  @brief  デフォルトコンストラクタ。Hogeオブジェクトを作成します
 */
Hoge::Hoge(){
    private_num = 0;
}

/**
 *  @fn     Hoge
 *  @brief  引数付きコンストラクタ。Hogeオブジェクトを作成します
 *  @param  初期化に指定する値
 */
Hoge::Hoge(int n){
    private_num = n;
}

/**
 *  @fn     Disp
 *  @brief  メンバ変数の表示
 */
void Hoge::Display(){
    std::cout << "num = " << private_num << std::endl;
}

/**
 *  @fn     ~Hoge
 *  @brief  デストラクタ
 */
Hoge::~Hoge(){}
