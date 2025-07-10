%%raw("import '@fontsource/inter'")

type params = {name: string}

@module("@tauri-apps/api/core")
external invoke: (string, params) => promise<string> = "invoke"

@react.component
let make = () => {
  React.useEffect(() => {
    let invokeGreet = async () => {
      try {
        let greeting = await invoke(
          "greet",
          {
            name: "Rescript",
          },
        )
        Js.Console.log(greeting)
      } catch {
      | Exn.Error(error) => Js.Console.error2("Error invoking greet", error)
      }
    }
    invokeGreet()->ignore
    None
  }, [])

  <Mui.ThemeProvider theme={Theme(Theme.theme)}>
    <Mui.CssBaseline />
    <MusicPlayer />
  </Mui.ThemeProvider>
}
