%%raw("import '@fontsource/inter'")

@react.component
let make = () => {
  <Mui.ThemeProvider theme={Theme(Theme.theme)}>
    <Mui.CssBaseline />
    <MusicPlayer />
  </Mui.ThemeProvider>
}
