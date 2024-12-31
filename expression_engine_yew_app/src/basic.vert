precision mediump float;

attribute vec2 a_position;
uniform mat3 u_transform;

//attribute vec4 a_position;
//uniform mat4 u_matrix;

void main() {
    vec3 position = vec3(a_position, 0.0);
    gl_Position = vec4(position, 1.0);

    //vec3 position = vec3(a_position, 0.0);
    //gl_Position = vec4(u_transform * position, 1.0);
}
