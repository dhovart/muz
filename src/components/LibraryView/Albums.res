@react.component
let make = (~albumsByArtist, ~onTrackSelect) => {
  let sortedArtists = React.useMemo(() => {
    albumsByArtist
    ->Js.Dict.keys
    ->Array.toSorted(String.compare)
  }, [albumsByArtist])

  sortedArtists
  ->Array.map(artist => {
    switch Js.Dict.get(albumsByArtist, artist) {
    | Some(artistAlbums) =>
      <Artists
        key={artist}
        artistAlbums
        onTrackSelect
        artistName={artist}
      />
    | None => React.null
    }
  })
  ->React.array
}
