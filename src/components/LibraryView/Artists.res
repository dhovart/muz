@react.component
let make = (~artistAlbums, ~onTrackSelect, ~artistName) => {
  let currentTrack = PlayerContext.usePlayer().currentTrack
  let sortedAlbums = React.useMemo(() => {
    artistAlbums
    ->Js.Dict.keys
    ->Array.toSorted(String.compare)
  }, [artistAlbums])

  <div className={LibraryViewStyles.artistGroup}>
    <h2 className={LibraryViewStyles.artistHeader}> {React.string(artistName)} </h2>
    {sortedAlbums
    ->Array.map(album => {
      switch Js.Dict.get(artistAlbums, album) {
      | Some(tracks) =>
        let handleAlbumTrackSelect = React.useCallback((track: Track.t) => {
          onTrackSelect(track, Some(album), Some(artistName))
        }, (album, artistName, onTrackSelect))

        <div key={album} className={LibraryViewStyles.albumGroup}>
          <h3> {React.string(album)} </h3>
          <TrackList tracks currentTrack onTrackSelect=handleAlbumTrackSelect context="library" />
        </div>
      | None => React.null
      }
    })
    ->React.array}
  </div>
}
