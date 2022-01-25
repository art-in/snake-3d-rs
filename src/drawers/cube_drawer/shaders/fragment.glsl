precision mediump float;

varying vec2 v_cube_texture_coord;
varying float v_cube_vertex_side;

// for covering different sides of cube with different textures we associate
// each vertex with side index, pass this side index from vertex shader to
// fragment shader through varying (even though it's the same for one side),
// and selecting correct texture basing on that side index. this allows us to
// draw entire cube with a single drawArrays call.
// another approach would be to draw different sides separately by rebinding
// same texture unit (TEXTURE0) to different textures, rebinding to different
// vertex buffers and calling drawArrays for each side (6 times).
// I guess single-draw approach should be faster, but not 100% sure which one is
// better overall (performance and code complexity wise)
// https://stackoverflow.com/questions/7767367/how-to-fill-each-side-of-a-cube-with-different-textures-on-opengl-es-1-1
uniform sampler2D u_cube_texture_side_0;
uniform sampler2D u_cube_texture_side_1;
uniform sampler2D u_cube_texture_side_2;
uniform sampler2D u_cube_texture_side_3;
uniform sampler2D u_cube_texture_side_4;
uniform sampler2D u_cube_texture_side_5;

void main() {
  // round cube side index. even though side index equals to the same integer
  // for each vertex of one cube side, and therefore should be the same for each
  // pixel of that cube side, for some reason it slightly deviates from integer.
  // most likely because of cumulative floating-point error while interpolating
  // varying between vertices
  int side = int(floor(v_cube_vertex_side + 0.5));

  if (side == 0) {
    gl_FragColor = texture2D(u_cube_texture_side_0, v_cube_texture_coord);
  } else
  if (side == 1) {
    gl_FragColor = texture2D(u_cube_texture_side_1, v_cube_texture_coord);
  } else
  if (side == 2) {
    gl_FragColor = texture2D(u_cube_texture_side_2, v_cube_texture_coord);
  } else
  if (side == 3) {
    gl_FragColor = texture2D(u_cube_texture_side_3, v_cube_texture_coord);
  } else
  if (side == 4) {
    gl_FragColor = texture2D(u_cube_texture_side_4, v_cube_texture_coord);
  } else
  if (side == 5) {
    gl_FragColor = texture2D(u_cube_texture_side_5, v_cube_texture_coord);
  }
}
