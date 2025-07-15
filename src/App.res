%%raw("import '@fontsource/inter'")

@module("@tauri-apps/api/core")
external invoke: (string, unit) => promise<unit> = "invoke"

type state = Playing | Paused | Stopped

@react.component
let make = () => {
  let (state, setState) = React.useState(() => Stopped)

  let invokePlayerCommand = async command => {
    switch command {
    | "previous_track" => setState(_ => Playing)
    | "next_track" => setState(_ => Playing)
    | "play" => setState(_ => state == Playing ? Paused : Playing)
    | _ => setState(_ => Stopped)
    }

    try {
      let ret = await invoke(command, ())
      Js.Console.log2("Player returned", ret)
    } catch {
    | Exn.Error(error) => Js.Console.error3("Error invoking player command", command, error)
    }
  }

  <Mui.ThemeProvider theme={Theme(Theme.theme)}>
    <Mui.CssBaseline />
    <MusicPlayer
      onPrev={() => invokePlayerCommand("previous_track")->ignore}
      onNext={() => invokePlayerCommand("next_track")->ignore}
      onPlayPause={() => invokePlayerCommand(state == Playing ? "pause" : "play")->ignore}
    />
  </Mui.ThemeProvider>
}
