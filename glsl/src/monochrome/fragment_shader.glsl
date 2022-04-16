#version 300 es
// ファイルの先頭にはバージョンを記述する必要がある
// コンパイラはこのバージョンを読み取りコンパイル処理を行う

// float（単制度浮動小数点）の精度
precision highp float;

// 画面に出力する色を格納する変数
out vec4 fragmentColor;

void main() {
    // Red, Green, Blue, Alpha
    fragmentColor = vec4(1.0, 0.0, 0.0, 1.0);
    // => 単色の赤色で塗りつぶされる
}
