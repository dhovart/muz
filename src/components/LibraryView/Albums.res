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
      <div key={album} className="album-group">
        <h3> {React.string(album)} </h3>
        <TrackList tracks currentTrack onTrackSelect context="library" />
      </div>
    | None => React.null
    }
  })
  ->React.array
}