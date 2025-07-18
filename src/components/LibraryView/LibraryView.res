open PlayerContext
open LibraryViewStyles

@react.component
let make = () => {
  let player = PlayerContext.usePlayer()

  let (tracks, setTracks) = React.useState(_ => [])
  let (loading, setLoading) = React.useState(_ => true)

  let loadTracks = React.useCallback0(() => {
    setLoading(_ => true)
    TrackService.getLibraryTracks()
    ->Promise.then(tracks => {
      setTracks(_ => tracks)
      setLoading(_ => false)
      Promise.resolve()
    })
    ->Promise.catch(error => {
      Js.Console.error2("Failed to load library tracks:", error)
      setLoading(_ => false)
      Promise.resolve()
    })
    ->ignore
  })

  let handleTrackSelect = React.useCallback0((track: Track.t) => {
    TrackService.playFromLibrary(track.id)
    ->Promise.then(_ => {
      Promise.resolve()
    })
    ->Promise.catch(error => {
      Js.Console.error2("Failed to prepend play from library:", error)
      Promise.resolve()
    })
    ->ignore
  })

  React.useEffect0(() => {
    loadTracks()
    None
  })

  <div className=container>
    <div className=header>
      <h2> {"Library"->React.string} </h2>
    </div>
    {loading
      ? <div>
          <Mui.CircularProgress />
        </div>
      : <TrackList
          tracks currentTrack=player.currentTrack onTrackSelect=handleTrackSelect context="library"
        />}
  </div>
}
