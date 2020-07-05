// 二重インクルード防止
#pragma once

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
