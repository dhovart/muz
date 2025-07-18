// Animation frame bindings
@val external requestAnimationFrame: (unit => unit) => int = "requestAnimationFrame"
@val external cancelAnimationFrame: int => unit = "cancelAnimationFrame"

// DOM bindings
@get external width: Dom.element => int = "width"
@get external height: Dom.element => int = "height"
@send external setAttribute: (Dom.element, string, string) => unit = "setAttribute"
type rectangle = {
  width: float,
  height: float,
}
@send
external getBoundingClientRect: Dom.element => rectangle = "getBoundingClientRect"

type glContext = {
  canvas: Dom.element,
  gl: WebGL.context,
  program: WebGL.program,
  buffer: WebGL.buffer,
  spectrumTexture: WebGL.texture,
  timeLocation: WebGL.uniformLocation,
  progressLocation: WebGL.uniformLocation,
  spectrumTextureLocation: WebGL.uniformLocation,
  resolutionLocation: WebGL.uniformLocation,
}

let vertexShaderSource = Shaders.vertexShader

let createShader = (gl: WebGL.context, type_: int, source: string): option<WebGL.shader> => {
  let shader = gl->WebGL.createShader(type_)
  gl->WebGL.shaderSource(shader, source)
  gl->WebGL.compileShader(shader)

  if gl->WebGL.getShaderParameter(shader, WebGL.compileStatus) {
    Some(shader)
  } else {
    Js.Console.error2("Shader compilation error:", gl->WebGL.getShaderInfoLog(shader))
    gl->WebGL.deleteShader(shader)
    None
  }
}

let createProgram = (
  gl: WebGL.context,
  vertexShader: WebGL.shader,
  fragmentShader: WebGL.shader,
): option<WebGL.program> => {
  let program = gl->WebGL.createProgram
  gl->WebGL.attachShader(program, vertexShader)
  gl->WebGL.attachShader(program, fragmentShader)
  gl->WebGL.linkProgram(program)

  if gl->WebGL.getProgramParameter(program, WebGL.linkStatus) {
    Some(program)
  } else {
    Js.Console.error2("Program linking error:", gl->WebGL.getProgramInfoLog(program))
    gl->WebGL.deleteProgram(program)
    None
  }
}

let initWebGL = (canvas: Dom.element, fragmentShaderSource: string): option<glContext> => {
  let gl = canvas->WebGL.getContextWithOptions({"antialias": true, "alpha": false})

  switch createShader(gl, WebGL.vertexShader, vertexShaderSource) {
  | None => None
  | Some(vertexShader) =>
    switch createShader(gl, WebGL.fragmentShader, fragmentShaderSource) {
    | None => None
    | Some(fragmentShader) =>
      switch createProgram(gl, vertexShader, fragmentShader) {
      | None => None
      | Some(program) => {
          // Create buffer for full-screen quad
          let buffer = gl->WebGL.createBuffer
          gl->WebGL.bindBuffer(WebGL.arrayBuffer, buffer)

          let vertices = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0]

          gl->WebGL.bufferData(
            WebGL.arrayBuffer,
            Float32Array.fromArray(vertices),
            WebGL.staticDraw,
          )

          // Create spectrum texture
          let spectrumTexture = gl->WebGL.createTexture
          gl->WebGL.bindTexture(WebGL.texture2D, spectrumTexture)
          gl->WebGL.texParameteri(WebGL.texture2D, WebGL.textureMinFilter, WebGL.nearest)
          gl->WebGL.texParameteri(WebGL.texture2D, WebGL.textureMagFilter, WebGL.nearest)

          // Get uniform locations
          let timeLocation = gl->WebGL.getUniformLocation(program, "u_time")
          let progressLocation = gl->WebGL.getUniformLocation(program, "u_progress")
          let spectrumTextureLocation = gl->WebGL.getUniformLocation(program, "u_spectrumTexture")
          let resolutionLocation = gl->WebGL.getUniformLocation(program, "u_resolution")

          Some({
            canvas,
            gl,
            program,
            buffer,
            spectrumTexture,
            timeLocation,
            progressLocation,
            spectrumTextureLocation,
            resolutionLocation,
          })
        }
      }
    }
  }
}

let render = (context: glContext, time: float, progress: float, spectrumData: array<float>) => {
  let gl = context.gl

  // Get actual canvas buffer dimensions
  let canvasWidth = context.canvas->width
  let canvasHeight = context.canvas->height

  // Set viewport
  gl->WebGL.viewport(0, 0, canvasWidth, canvasHeight)

  // Clear canvas
  gl->WebGL.clearColor(0.0, 0.0, 0.0, 1.0)
  gl->WebGL.clear(WebGL.colorBufferBit)

  // Use program
  gl->WebGL.useProgram(context.program)

  // Set up attributes
  let positionLocation = gl->WebGL.getAttribLocation(context.program, "a_position")
  gl->WebGL.bindBuffer(WebGL.arrayBuffer, context.buffer)
  gl->WebGL.enableVertexAttribArray(positionLocation)
  gl->WebGL.vertexAttribPointer(positionLocation, 2, WebGL.float, false, 0, 0)

  // Set uniforms
  gl->WebGL.uniform1f(context.timeLocation, time)
  gl->WebGL.uniform1f(context.progressLocation, progress)
  gl->WebGL.uniform2f(
    context.resolutionLocation,
    canvasWidth->Int.toFloat,
    canvasHeight->Int.toFloat,
  )
  let copyLen = Js.Math.min_int(Array.length(spectrumData), 64)

  // Update spectrum texture
  let paddedSpectrum = Array.make(~length=64, 0)
  for i in 0 to copyLen - 1 {
    let value = switch spectrumData[i] {
    | Some(value) => value
    | None => 0.0
    }
    let intValue = Int.fromFloat(value *. 255.0)
    paddedSpectrum[i] = intValue
  }

  // Upload texture data
  gl->WebGL.activeTexture(WebGL.texture0)
  gl->WebGL.bindTexture(WebGL.texture2D, context.spectrumTexture)
  gl->WebGL.texImage2D(
    WebGL.texture2D,
    0,
    WebGL.luminance,
    64,
    1,
    0,
    WebGL.luminance,
    WebGL.unsignedByte,
    Uint8Array.fromArray(paddedSpectrum),
  )
  gl->WebGL.uniform1i(context.spectrumTextureLocation, 0)

  // Draw
  gl->WebGL.drawArrays(WebGL.triangles, 0, 6)
}

@react.component
let make = (
  ~progress: float,
  ~spectrumData: array<float>,
  ~width: int=640,
  ~height: int=200,
  ~className: string="",
  ~fragmentShader: string,
) => {
  let canvasRef = React.useRef(Nullable.null)
  let contextRef = React.useRef(None)
  let animationRef = React.useRef(None)
  let startTimeRef = React.useRef(None)
  let progressRef = React.useRef(progress)
  let spectrumDataRef = React.useRef(spectrumData)
  let fragmentShaderRef = React.useRef(fragmentShader)

  let updateCanvasSize = () => {
    switch canvasRef.current->Nullable.toOption {
    | None => ()
    | Some(canvas) => {
        let rect = canvas->getBoundingClientRect
        let displayWidth = Float.toInt(rect.width)
        let displayHeight = Float.toInt(rect.height)
        let devicePixelRatio = %raw(`window.devicePixelRatio || 1.0`)

        let actualWidth = (Int.toFloat(displayWidth) *. devicePixelRatio)->Float.toInt
        let actualHeight = (Int.toFloat(displayHeight) *. devicePixelRatio)->Float.toInt

        if actualWidth !== width || actualHeight !== height {
          canvas->setAttribute("width", actualWidth->Int.toString)
          canvas->setAttribute("height", actualHeight->Int.toString)
        }
      }
    }
  }

  // Update refs when props change
  React.useEffect(() => {
    progressRef.current = progress
    spectrumDataRef.current = spectrumData
    fragmentShaderRef.current = fragmentShader

    // Force a single render with new data
    switch contextRef.current {
    | None => ()
    | Some(context) => {
        let now = Js.Date.now()
        let startTime = switch startTimeRef.current {
        | None => now
        | Some(t) => t
        }
        let time = (now -. startTime) /. 1000.0
        render(context, time, progress, spectrumData)
      }
    }
    None
  }, (progress, spectrumData, fragmentShader))

  React.useEffect(() => {
    switch canvasRef.current->Nullable.toOption {
    | None => ()
    | Some(canvas) =>
      switch initWebGL(canvas, fragmentShader) {
      | None => Js.Console.error("Failed to initialize WebGL")
      | Some(context) => {
          contextRef.current = Some(context)
          let rec animate = () => {
            updateCanvasSize()

            let now = Js.Date.now()
            let startTime = switch startTimeRef.current {
            | None => {
                startTimeRef.current = Some(now)
                now
              }
            | Some(t) => t
            }

            let time = (now -. startTime) /. 1000.0
            render(context, time, progressRef.current, spectrumDataRef.current)

            animationRef.current = Some(requestAnimationFrame(animate))
          }

          animate()
        }
      }
    }

    Some(
      () => {
        switch animationRef.current {
        | None => ()
        | Some(id) => cancelAnimationFrame(id)
        }
      },
    )
  }, [fragmentShader])

  <canvas
    ref={ReactDOM.Ref.domRef(canvasRef)}
    width={width->Int.toString}
    height={height->Int.toString}
    className={className}
    style={{width: "100%", height: "100%"}}
  />
}
