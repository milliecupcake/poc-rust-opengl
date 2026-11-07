#version 410 core

in vec3 v_normal;
in vec3 v_color;

out vec4 frag_color;

const vec3 LIGHT_DIRECTION = normalize(vec3(0.5, 0.8, 0.6));
const float AMBIENT = 0.3;

void main() {
    float diffuse = max(dot(normalize(v_normal), LIGHT_DIRECTION), 0.0);
    frag_color = vec4(v_color * (AMBIENT + (1.0 - AMBIENT) * diffuse), 1.0);
}
