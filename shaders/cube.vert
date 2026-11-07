#version 410 core

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec3 a_color;

uniform mat4 u_model;
uniform mat4 u_view_projection;

out vec3 v_normal;
out vec3 v_color;

void main() {
    // モデル行列は回転のみなので、法線にもそのまま適用できる。
    v_normal = mat3(u_model) * a_normal;
    v_color = a_color;
    gl_Position = u_view_projection * u_model * vec4(a_position, 1.0);
}
