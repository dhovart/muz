%%raw("import '@fontsource/inter'")

@module("@tauri-apps/api/core")
external invoke: (string, unit) => promise<unit> = "invoke"

@react.component
let make = () => {
  let invokePlay = async () => {
    try {
      await invoke("play", ())
    } catch {
    | Exn.Error(error) => Js.Console.error2("Error invoking play", error)
    }
  }

  <Mui.ThemeProvider theme={Theme(Theme.theme)}>
    <Mui.CssBaseline />
    <MusicPlayer onPlayPause={() => invokePlay()->ignore} />
  </Mui.ThemeProvider>
}
