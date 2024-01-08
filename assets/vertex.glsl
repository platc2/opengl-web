#version 300 es
precision highp float;

layout (location = 0) in vec2 vertex;
layout (location = 1) in vec3 color;

out vec3 fragment_color;

void main(void) {
    gl_Position = vec4(vertex, 0, 1);
    fragment_color = color;
}

