%%raw("import '@fontsource/inter'")

open PlayerContext
open State

@react.component
let make = () => {
  let (currentPage, setCurrentPage) = React.useState(() => Route.MusicPlayer)
  let (isQueueDrawerOpen, setIsQueueDrawerOpen) = React.useState(() => false)

  let handlePageChange = (newPage: Route.t) => {
    setCurrentPage(_ => newPage)
  }

  let handleQueueToggle = () => {
    setIsQueueDrawerOpen(isOpen => !isOpen)
  }

  let handleQueueClose = () => {
    setIsQueueDrawerOpen(_ => false)
  }

  let (state, dispatch) = PlayerContext.usePlayerReducer()

  <Mui.ThemeProvider theme={Theme(Theme.theme)}>
    <Mui.CssBaseline />
    <PlayerProvider state={state} dispatch={dispatch}>
      <div>
        {currentPage !== Route.Visualizer
          ? <Menu
              currentPage={currentPage}
              onPageChange={handlePageChange}
              onQueueToggle={handleQueueToggle}
            />
          : React.null}
        {switch currentPage {
        | Route.MusicPlayer => <MusicPlayerPage />
        | Route.Library => <LibraryPage />
        | Route.Settings => <SettingsPage />
        | Route.Visualizer => <VisualizerPage onExit={() => handlePageChange(Route.MusicPlayer)} />
        }}
        <QueueDrawer isOpen={isQueueDrawerOpen} onClose={handleQueueClose} />
      </div>
    </PlayerProvider>
  </Mui.ThemeProvider>
}
