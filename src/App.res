%%raw("import '@fontsource/inter'")

@react.component
let make = () => {
  let (currentPage, setCurrentPage) = React.useState(() => Route.MusicPlayer)

  let handlePageChange = (newPage: Route.t) => {
    setCurrentPage(_ => newPage)
  }

  <Mui.ThemeProvider theme={Theme(Theme.theme)}>
    <Mui.CssBaseline />
    <div>
      <Menu currentPage={currentPage} onPageChange={handlePageChange} />
      {switch currentPage {
      | Route.MusicPlayer => <MusicPlayerPage />
      | Route.Settings => <SettingsPage />
      }}
    </div>
  </Mui.ThemeProvider>
}
