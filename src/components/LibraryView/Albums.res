@react.component
let make = (~albumsMap, ~currentTrack, ~onTrackSelect) => {
  let sortedAlbums = React.useMemo(() => {
    albumsMap
    ->Js.Dict.keys
    ->Array.toSorted(String.compare)
  }, [albumsMap])

  sortedAlbums
  ->Array.map(album => {
    switch Js.Dict.get(albumsMap, album) {
    | Some(tracks) =>
      let handleAlbumTrackSelect = React.useCallback((track: Track.t) => {
        let artist = switch tracks->Array.get(0) {
        | Some(firstTrack: Track.t) =>
          switch firstTrack.metadata.artist {
          | Some(artistName) => Some(artistName)
          | None => firstTrack.metadata.album_artist
          }
        | None => None
        }
        onTrackSelect(track, Some(album), artist)
      }, (tracks, album, onTrackSelect))

      <div key={album} className="album-group">
        <h3> {React.string(album)} </h3>
        <TrackList tracks currentTrack onTrackSelect=handleAlbumTrackSelect context="library" />
      </div>
    | None => React.null
    }
  })
  ->React.array
}
