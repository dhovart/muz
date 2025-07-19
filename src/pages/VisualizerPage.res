open UseSpectrumData

@react.component
let make = (~onExit: unit => unit=() => ()) => {
  let spectrumData = useSpectrumData()
  let (selectedShader, setSelectedShader) = React.useState(() => "colorful_thingy")
  let (shaderSource, setShaderSource) = React.useState(() => Shaders.simpleFragmentShader)
  let (showControls, setShowControls) = React.useState(() => true)
  let (hideTimeout, setHideTimeout) = React.useState(() => None)

  let shaderOptions = [("default", "Default"), ("colorful_thingy", "Colorful Thingy")]

  let loadShader = (shaderName: string) => {
    switch shaderName {
    | "colorful_thingy" => setShaderSource(_ => Shaders.colorfulThingyFragmentShader)
    | "default"
    | _ =>
      setShaderSource(_ => Shaders.defaultFragmentShader)
    }
  }

  // Load initial shader
  React.useEffect(() => {
    loadShader(selectedShader)
    None
  }, [selectedShader])

  // Auto-hide controls after 3 seconds of inactivity
  let resetHideTimer = () => {
    switch hideTimeout {
    | Some(timeoutId) => Js.Global.clearTimeout(timeoutId)
    | None => ()
    }

    setShowControls(_ => true)
    let newTimeout = Js.Global.setTimeout(() => setShowControls(_ => false), 3000)
    setHideTimeout(_ => Some(newTimeout))
  }

  // Show controls on mouse move
  React.useEffect(() => {
    resetHideTimer()

    Some(
      () => {
        switch hideTimeout {
        | Some(timeoutId) => Js.Global.clearTimeout(timeoutId)
        | None => ()
        }
      },
    )
  }, [])

  <div
    onMouseMove={_ => resetHideTimer()}
    style={{
      position: "fixed",
      top: "0",
      left: "0",
      width: "100vw",
      height: "100vh",
      overflow: "hidden",
      backgroundColor: "#000",
      zIndex: "1000",
    }}>
    <Visualizer
      spectrumData={spectrumData}
      fragmentShader={shaderSource}
      width={1920}
      height={1080}
      className=""
    />
    {showControls
      ? <div
          style={{
            position: "absolute",
            top: "20px",
            right: "20px",
            background: "rgba(0, 0, 0, 0.8)",
            borderRadius: "8px",
            padding: "16px",
            color: "white",
            fontFamily: "Inter, sans-serif",
            fontSize: "14px",
            zIndex: "1001",
          }}>
          <div
            style={{
              marginBottom: "12px",
              fontWeight: "bold",
              display: "flex",
              justifyContent: "space-between",
              alignItems: "center",
            }}>
            <span> {React.string("Visualizer")} </span>
            <button
              onClick={_ => onExit()}
              style={{
                background: "rgba(255, 255, 255, 0.2)",
                border: "none",
                borderRadius: "4px",
                color: "white",
                padding: "4px 8px",
                fontSize: "12px",
                cursor: "pointer",
              }}>
              {React.string("Exit")}
            </button>
          </div>
          <div style={{marginBottom: "8px"}}> {React.string("Shader:")} </div>
          <select
            value={selectedShader}
            onChange={e => {
              let target = e->ReactEvent.Form.target
              let value = target["value"]
              setSelectedShader(_ => value)
            }}
            style={{
              background: "rgba(255, 255, 255, 0.1)",
              border: "1px solid rgba(255, 255, 255, 0.3)",
              borderRadius: "4px",
              color: "white",
              padding: "4px 8px",
              width: "150px",
              marginBottom: "12px",
            }}>
            {shaderOptions
            ->Array.map(((value, label)) =>
              <option key={value} value={value}> {React.string(label)} </option>
            )
            ->React.array}
          </select>
        </div>
      : React.null}
  </div>
}
