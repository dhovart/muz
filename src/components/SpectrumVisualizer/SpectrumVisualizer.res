open Mui

// Canvas bindings
module Canvas = {
  type t
  type context2d

  @send external getContext: (t, string) => Nullable.t<context2d> = "getContext"
  @get external width: t => string = "width"
  @get external height: t => string = "height"

  @send external clearRect: (context2d, float, float, float, float) => unit = "clearRect"
  @send external fillRect: (context2d, float, float, float, float) => unit = "fillRect"
  @set external setFillStyle: (context2d, string) => unit = "fillStyle"
  @set external setStrokeStyle: (context2d, string) => unit = "strokeStyle"
  @set external setLineWidth: (context2d, float) => unit = "lineWidth"
}

// Animation frame bindings
@val external requestAnimationFrame: (unit => unit) => float = "requestAnimationFrame"
@val external cancelAnimationFrame: float => unit = "cancelAnimationFrame"

@react.component
let make = (~spectrumData: array<float>) => {
  let canvasRef = React.useRef(Js.Nullable.null)
  let animationFrameRef = React.useRef(0.0)
  let lastSpectrumDataRef = React.useRef(spectrumData)

  let drawSpectrum = React.useCallback(() => {
    switch Js.Nullable.toOption(canvasRef.current) {
    | Some(domElement) =>
      let canvas = (Obj.magic(domElement): Canvas.t)
      switch canvas->Canvas.getContext("2d")->Nullable.toOption {
      | Some(ctx) => {
          let width = Js.Float.fromString(Canvas.width(canvas))
          let height = Js.Float.fromString(Canvas.height(canvas))

          Canvas.clearRect(ctx, 0.0, 0.0, width, height)

          let currentData = lastSpectrumDataRef.current
          if Array.length(currentData) > 0 {
            // Set up drawing style
            Canvas.setFillStyle(ctx, "#4fc3f7")
            Canvas.setStrokeStyle(ctx, "#29b6f6")
            Canvas.setLineWidth(ctx, 1.0)

            let barWidth = width /. Float.fromInt(Array.length(currentData))
            let maxHeight = height *. 0.8

            // Draw spectrum bars
            Array.forEachWithIndex(currentData, (value, index) => {
              let normalizedValue = Js.Math.max_float(0.0, Js.Math.min_float(1.0, value *. 100.0))
              let barHeight = normalizedValue *. maxHeight
              let x = Float.fromInt(index) *. barWidth
              let y = height -. barHeight

              Canvas.fillRect(ctx, x, y, barWidth -. 1.0, barHeight)
            })
          }
        }
      | None => ()
      }
    | None => ()
    }
  }, [])

  React.useEffect(() => {
    lastSpectrumDataRef.current = spectrumData

    // Cancel any existing animation frame
    if animationFrameRef.current !== 0.0 {
      cancelAnimationFrame(animationFrameRef.current)
    }

    // Schedule the next render
    animationFrameRef.current = requestAnimationFrame(() => {
      drawSpectrum()
      animationFrameRef.current = 0.0
    })

    // Cleanup function
    Some(
      () => {
        if animationFrameRef.current !== 0.0 {
          cancelAnimationFrame(animationFrameRef.current)
        }
      },
    )
  }, [spectrumData])

  <canvas ref={canvasRef->ReactDOM.Ref.domRef} width="320.0" height="100.0" />
}
