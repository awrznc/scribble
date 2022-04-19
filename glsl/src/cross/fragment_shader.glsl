#version 300 es

precision highp float;
uniform float u_time;

// resolution: スクリーンのサイズ
//  - x: width (example: 100)
//  - y: height (example: 100)
uniform vec2 u_resolution;

out vec4 fragmentColor;

void main(void){

    // gl_FragCoord: vec4型のデータ
    //  - x: 処理対象のピクセルの横位置 (example: 0 ~ 100)
    //  - y: 処理対象のピクセルの縦位置 (example: 0 ~ 100)

    // 中心が0となるように範囲をずらす
    // 0 ~ 100 -> -100 ~ 100
    //  - (0 ~ 100) の座標にそれぞれ2をかけることで (0 ~ 200) にする
    //  - それぞれの座標の値から横幅(100)を引くと(-100 ~ 100) になる
    vec2 shifted_data = (gl_FragCoord.xy * 2.0 - u_resolution);

    // xかy小さいほうを基準に-1.0 ~ 1.0 の範囲に整形する
    vec2 normalized_data = shifted_data / min(u_resolution.x, u_resolution.y);

    // lengthで原点からの距離を取得する
    float x = 1.0 / length(normalized_data.x);
    float y = 1.0 / length(normalized_data.y);

    // 乗算
    // 大きくなる横軸yと大きくなる縦軸xが合わさり光の十字が表示される
    float rgb = x * y * abs(sin(u_time));

    fragmentColor = vec4(vec3(rgb), 1.0);
}
