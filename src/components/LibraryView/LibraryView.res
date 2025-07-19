open PlayerContext
open LibraryViewStyles

@react.component
let make = () => {
  let player = PlayerContext.usePlayer()

  let (albumsMap, setAlbumsMap) = React.useState(_ => Js.Dict.empty())
  let (loading, setLoading) = React.useState(_ => true)

  let loadTracks = React.useCallback0(() => {
    setLoading(_ => true)
    TrackService.getLibraryTracks()
    ->Promise.then(albumsMap => {
      setAlbumsMap(_ => albumsMap)
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

  let handleTrackSelect = React.useCallback0((
    track: Track.t,
    album: option<string>,
    artist: option<string>,
  ) => {
    TrackService.playFromLibrary(track.id, ~album, ~artist, ())->ignore
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
      : <Albums albumsMap currentTrack=player.currentTrack onTrackSelect=handleTrackSelect />}
  </div>
}
