// $ g++ main.cpp -o app && ./app.exe

#include<stdio.h>
#include<iostream>
#include<assert.h>

// ===== 標準出力 ===== ===== =====
void print_hello() {

    // Dependence: stdio
    printf("Hello, World!\n");

    // Dependence: iostream
    std::cout << "Hello, World!\n";
}


// ===== 参照型 ===== ===== =====
// 変数に別名を付けて参照することが可能となる機能
// 利点: ポインタのように指し先が曖昧になることがない点。NULLポインタのような特殊な状態を持つこともない
void reference() {

    int x;

    // yをint型への参照として定義する
    // 変数のアドレスを出力する「&」演算子とは異なるので注意
    int &y = x;

    // 初期化後にアドレスを変更することは不可
    // error: lvalue required as left operand of assignment
    // &y = &x;

    // 代入は可能
    y = 30;
    assert(x == 30);
    assert(y == 30);

    // ポインタで変数を参照することも可能
    int *z = &x;
    assert(*z == 30);
}


// ===== 名前空間 ===== ===== =====
// 名前の衝突を防ぐためのもの
namespace sample {
    int sample_fn() { return 1 + 1; }

    // 名前空間の中では修飾なしで呼び出せる
    int wrap_sample() { return sample_fn(); }

    // 入れ子にもできる
    namespace nest {
        int wrap_sample() { return sample_fn(); }
    }
}

// Using宣言を行うことで冗長な記述を簡略化できる
using namespace sample;

// Using宣言でnamespaceを利用すると影響範囲が大きくなり、衝突する懸念がある
// どうしても利用したい場合は、以下のような形でスコープを刻んだ方がよい
using sample::sample_fn ;

void name_space() {
    assert(sample::sample_fn() == 2);
    assert(sample::wrap_sample() == 2);
    assert(sample::nest::wrap_sample() == 2);

    // 既存の名前空間に別名を付けることもできる
    namespace sn = sample::nest;
    assert(sn::wrap_sample() == 2);

    // Using宣言が有効になっているかの確認
    assert(wrap_sample() == 2);
    assert(sample_fn() == 2);
}


// ===== class ===== ===== =====
class SampleClass {
    public:
        int public_value;

        // constructor
        SampleClass(int value) {
            public_value = private_value = value;
        }

        // destructor
        ~SampleClass() {}

        int get_value();
    private:
        int private_value;
};

int SampleClass::get_value() {
    return this->public_value;
}

void class_sample() {
    int value = 2;

    // new
    SampleClass instance(value);

    // method
    assert(instance.get_value() == value);

    // public variable
    assert(instance.public_value == value);

    // private variable
    // error: 'int SampleClass::private_value' is private
    // assert(instance.private_value == value);

}

// ===== main ===== ===== =====
int main() {
    print_hello();
    reference();
    name_space();
    class_sample();
}
