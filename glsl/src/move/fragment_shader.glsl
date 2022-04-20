#version 300 es

precision highp float;
uniform float u_time;
uniform vec2 u_resolution;
out vec4 fragmentColor;

void main(void){

    vec2 shifted_data = (gl_FragCoord.xy * 2.0 - u_resolution);
    vec2 normalized_data = shifted_data / min(u_resolution.x, u_resolution.y);

    // xをsinの周期、yをcosの周期で画面の中心を基準に移動させる
    normalized_data.x += sin(u_time);
    normalized_data.y += cos(u_time);
    float rgb = 0.1 / length(normalized_data.xy);

    fragmentColor = vec4(vec3(rgb), 1.0);
}
