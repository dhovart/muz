open LibraryViewStyles

@react.component
let make = () => {
  let (albumsByArtist, setAlbumsByArtist) = React.useState(_ => Js.Dict.empty())
  let (loading, setLoading) = React.useState(_ => true)

  let loadTracks = React.useCallback0(() => {
    setLoading(_ => true)
    LibraryService.getAlbumsByArtist()
    ->Promise.then(albumsByArtist => {
      setAlbumsByArtist(_ => albumsByArtist)
      setLoading(_ => false)
      Promise.resolve()
    })
    ->Promise.catch(error => {
      Js.Console.error2("Failed to load albums by artist:", error)
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
    PlaybackService.playFromLibrary(track.id, ~album, ~artist, ())->ignore
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
      : <Albums albumsByArtist onTrackSelect=handleTrackSelect />}
  </div>
}
