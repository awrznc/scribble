#version 300 es

precision highp float;
uniform float u_time;
out vec4 fragmentColor;

void main() {

    // sin ( 0 -> 1 -> 0 -> -1 -> 0 -> 1 ... ) をabsで絶対値に変換する
    float red = abs(sin(u_time));

    fragmentColor = vec4(red, 0.0, 0.0, 1.0);
    // => 赤色がゆっくりと点滅する
}
