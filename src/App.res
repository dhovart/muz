%%raw("import '@fontsource/inter'")

open PlayerContext
open AppStyles

@react.component
let make = () => {
  let (currentPage, setCurrentPage) = React.useState(() => Route.Library)
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
    <Mui.CssBaseline enableColorScheme=true />
    <PlayerProvider state={state} dispatch={dispatch}>
      <div className={appContainer}>
        {currentPage !== Route.Visualizer
          ? <Menu
              currentPage={currentPage}
              onPageChange={handlePageChange}
              onQueueToggle={handleQueueToggle}
            />
          : React.null}
        <div className={contentArea}>
          {switch currentPage {
          | Route.Library => <LibraryPage />
          | Route.MillerColumns => <MillerColumnsView />
          | Route.Settings => <SettingsPage />
          | Route.Visualizer => <VisualizerPage onExit={() => handlePageChange(Route.Library)} />
          }}
        </div>
        <MusicPlayer />
        <QueueDrawer isOpen={isQueueDrawerOpen} onClose={handleQueueClose} />
      </div>
    </PlayerProvider>
  </Mui.ThemeProvider>
}
