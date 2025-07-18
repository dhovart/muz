type context
type shader
type program
type buffer
type texture
type uniformLocation
type attribLocation = int

// WebGL constants
let vertexShader = 35633
let fragmentShader = 35632
let arrayBuffer = 34962
let staticDraw = 35044
let triangles = 4
let float = 5126
let colorBufferBit = 16384
let compileStatus = 35713
let linkStatus = 35714
let texture2D = 3553
let textureMinFilter = 10241
let textureMagFilter = 10240
let nearest = 9728
let luminance = 6409
let unsignedByte = 5121
let texture0 = 33984

// Canvas and context
@send external getContext: (Dom.element, @as("webgl") _) => context = "getContext"
@send external getContextWithOptions: (Dom.element, @as("webgl") _, 'options) => context = "getContext"

// Shader operations
@send external createShader: (context, int) => shader = "createShader"
@send external shaderSource: (context, shader, string) => unit = "shaderSource"
@send external compileShader: (context, shader) => unit = "compileShader"
@send external getShaderParameter: (context, shader, int) => bool = "getShaderParameter"
@send external getShaderInfoLog: (context, shader) => string = "getShaderInfoLog"
@send external deleteShader: (context, shader) => unit = "deleteShader"

// Program operations
@send external createProgram: context => program = "createProgram"
@send external attachShader: (context, program, shader) => unit = "attachShader"
@send external linkProgram: (context, program) => unit = "linkProgram"
@send external getProgramParameter: (context, program, int) => bool = "getProgramParameter"
@send external getProgramInfoLog: (context, program) => string = "getProgramInfoLog"
@send external deleteProgram: (context, program) => unit = "deleteProgram"
@send external useProgram: (context, program) => unit = "useProgram"

// Buffer operations
@send external createBuffer: context => buffer = "createBuffer"
@send external bindBuffer: (context, int, buffer) => unit = "bindBuffer"
@send external bufferData: (context, int, Float32Array.t, int) => unit = "bufferData"

// Uniform and attribute operations
@send external getUniformLocation: (context, program, string) => uniformLocation = "getUniformLocation"
@send external getAttribLocation: (context, program, string) => attribLocation = "getAttribLocation"
@send external uniform1f: (context, uniformLocation, float) => unit = "uniform1f"
@send external uniform2f: (context, uniformLocation, float, float) => unit = "uniform2f"
@send external uniform1fv: (context, uniformLocation, Float32Array.t) => unit = "uniform1fv"

// Vertex attributes
@send external enableVertexAttribArray: (context, attribLocation) => unit = "enableVertexAttribArray"
@send external vertexAttribPointer: (context, attribLocation, int, int, bool, int, int) => unit = "vertexAttribPointer"

// Texture operations
@send external createTexture: context => texture = "createTexture"
@send external bindTexture: (context, int, texture) => unit = "bindTexture"
@send external texImage2D: (context, int, int, int, int, int, int, int, int, Uint8Array.t) => unit = "texImage2D"
@send external texParameteri: (context, int, int, int) => unit = "texParameteri"
@send external activeTexture: (context, int) => unit = "activeTexture"
@send external uniform1i: (context, uniformLocation, int) => unit = "uniform1i"

// Rendering
@send external viewport: (context, int, int, int, int) => unit = "viewport"
@send external clearColor: (context, float, float, float, float) => unit = "clearColor"
@send external clear: (context, int) => unit = "clear"
@send external drawArrays: (context, int, int, int) => unit = "drawArrays"

// Float32Array
module Float32Array = {
  type t
  @new external fromArray: array<float> => t = "Float32Array"
}

// Uint8Array
module Uint8Array = {
  type t
  @new external fromArray: array<int> => t = "Uint8Array"
}

