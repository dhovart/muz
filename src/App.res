%%raw("import '@fontsource/inter'")

open Command
@module("@tauri-apps/api/core")
external invoke: (string, Js.Json.t) => promise<unit> = "invoke"

// // FIXME make a functor and return different types instead of using Js.Json.t
// type channelEvent = {data: Js.Json.t}
// type channelType = {mutable onmessage: Js.Json.t => unit}
// @module("@tauri-apps/api/core") @new external channel: unit => channelType = "Channel"

type state = Playing | Paused | Stopped

@react.component
let make = () => {
  open Command
  let (state, setState) = React.useState(() => Stopped)
  let (volume, setVolume) = React.useState(() => 0.5)

  let invokePlayerCommand = async command => {
    switch command {
    | Play => setState(_ => Playing)
    | Next => setState(_ => Playing)
    | Previous => setState(_ => Playing)
    | Pause => setState(_ => state == Paused ? Playing : Paused)
    | SetVolume(vol) => setVolume(_ => vol)
    }

    let payload = toJsonPayload(command)

    try {
      let ret = await invoke("control_playback", payload)
      Js.Console.log2("Player returned", ret)
    } catch {
    | Exn.Error(error) => Js.Console.error3("Error invoking player command", payload, error)
    }
  }

  React.useEffect(() => {
    invokePlayerCommand(SetVolume(volume))->ignore
    None
  }, [volume])

  // React.useEffect(() => {
  //   let onEvent = channel()
  //   onEvent.onmessage = message => {
  //     Js.Console.log(`got download event` ++ Js.Json.stringify(message))
  //   }

  //   None
  // }, [])

  <Mui.ThemeProvider theme={Theme(Theme.theme)}>
    <Mui.CssBaseline />
    <Mui.Slider
      value={volume}
      min={0.0}
      max={1.0}
      size=Medium
      step={Number(0.01)}
      orientation=Horizontal
      onChange={(_, value, _) => setVolume(_ => value)->ignore}
      ariaLabel="Volume"
    />
    <MusicPlayer
      onPrev={() => invokePlayerCommand(Previous)->ignore}
      onNext={() => invokePlayerCommand(Next)->ignore}
      onPlayPause={() => invokePlayerCommand(state == Playing ? Pause : Play)->ignore}
    />
  </Mui.ThemeProvider>
}
