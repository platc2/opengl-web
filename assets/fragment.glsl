#version 300 es
precision highp float;

in vec3 fragment_color;

layout (location = 0) out vec4 color;

/*
layout (std140, binding = 0) uniform Data {
    float gamma;
};
*/

void main(void) {
    color = vec4(fragment_color, 1);
//    color.rgb = pow(color.rgb, vec3(1 / gamma));
}
