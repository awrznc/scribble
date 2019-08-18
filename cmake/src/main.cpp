#include <iostream>

/**
 *  @class  Hoge
 *  @brief  int型の値を扱うクラスです
 */
class Hoge {
private:
    int private_num;

public:
    Hoge();
    Hoge(int n);
    void Display();
    ~Hoge();
};

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
