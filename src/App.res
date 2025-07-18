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

  let playerReducer = (state: playerState, action: playerAction) => {
    switch action {
    | SetCurrentTrack(track) => {...state, currentTrack: track}
    | SetQueue(queue) => {...state, queue}
    | SetHasHistory(hasHistory) => {...state, hasHistory}
    | SetPosition(position) => {...state, position}
    | SetVolume(volume) => {...state, volume}
    | SetState(playerState) => {...state, state: playerState}
    | SetSpectrumData(spectrumData) => {...state, spectrumData}
    }
  }

  let (state, dispatch) = React.useReducer(
    playerReducer,
    {
      currentTrack: None,
      queue: [],
      hasHistory: false,
      position: 0.0,
      volume: 0.5,
      state: State.Stopped,
      spectrumData: [],
    },
  )

  let handleTrackSelect = (track: Track.t) => {
    // TODO: Add track selection logic
    Js.log2("Track selected:", track)
  }

  <Mui.ThemeProvider theme={Theme(Theme.theme)}>
    <Mui.CssBaseline />
    <PlayerProvider state={state} dispatch={dispatch}>
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
    </PlayerProvider>
  </Mui.ThemeProvider>
}
