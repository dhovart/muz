%%raw("import '@fontsource/inter'")

@react.component
let make = () => {
  let (currentPage, setCurrentPage) = React.useState(() => Route.MusicPlayer)
  let (isQueueDrawerOpen, setIsQueueDrawerOpen) = React.useState(() => false)

  let handlePageChange = (newPage: Route.t) => {
    setCurrentPage(_ => newPage)
  }

  let handleQueueToggle = () => {
    setIsQueueDrawerOpen(prev => !prev)
  }

  let handleQueueClose = () => {
    setIsQueueDrawerOpen(_ => false)
  }

  let playerReducer = (state: PlayerContext.playerState, action: PlayerContext.playerAction) => {
    switch action {
    | SetCurrentTrack(track) => {...state, currentTrack: track}
    | SetQueue(queue) => {...state, queue}
    }
  }

  let (state, dispatch) = React.useReducer(
    playerReducer,
    {
      currentTrack: None,
      queue: [],
    },
  )

  let handleTrackSelect = (track: Track.t) => {
    // TODO: Add track selection logic
    Js.log2("Track selected:", track)
  }

  <Mui.ThemeProvider theme={Theme(Theme.theme)}>
    <Mui.CssBaseline />
    <PlayerContext.PlayerProvider state={state} dispatch={dispatch}>
      <div>
        <Menu
          currentPage={currentPage}
          onPageChange={handlePageChange}
          onQueueToggle={handleQueueToggle}
        />
        {switch currentPage {
        | Route.MusicPlayer => <MusicPlayerPage />
        | Route.Library => <LibraryPage />
        | Route.Settings => <SettingsPage />
        }}
        <QueueDrawer
          isOpen={isQueueDrawerOpen} onClose={handleQueueClose} onTrackSelect={handleTrackSelect}
        />
      </div>
    </PlayerContext.PlayerProvider>
  </Mui.ThemeProvider>
}
