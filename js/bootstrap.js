import vertexShaderSrc from '../shaders/vertex.glsl';
import fragmentShaderSrc from '../shaders/fragment.glsl';

// a dependency graph that contains any wasm must all be imported asynchronously
import("../pkg/index.js")
  .then(module => module.init(vertexShaderSrc, fragmentShaderSrc))
  .catch(console.error);


