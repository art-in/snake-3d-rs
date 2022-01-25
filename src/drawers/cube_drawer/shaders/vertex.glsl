attribute vec4 a_cube_vertex_coord;
attribute vec2 a_cube_texture_coord;
attribute float a_cube_vertex_side;

uniform mat4 u_matrix;

varying vec2 v_cube_texture_coord;
varying float v_cube_vertex_side;

void main() {
  gl_Position = u_matrix * a_cube_vertex_coord;

  v_cube_texture_coord = a_cube_texture_coord;
  v_cube_vertex_side = a_cube_vertex_side;
}
