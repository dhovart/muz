@react.component
let make = (
  ~title: string,
  ~items: array<(string, string)>,
  ~selectedItem: option<string>,
  ~onSelect: string => unit,
  ~currentTrack: option<Track.t>,
) => {
  <div className={MillerColumnsViewStyles.column}>
    <div className={MillerColumnsViewStyles.columnHeader}> {React.string(title)} </div>
    <div className={MillerColumnsViewStyles.columnContent}>
      {items
      ->Belt.Array.map(((displayName, value)) => {
        let isSelected = selectedItem === Some(value)
        let isCurrentTrackRelated = switch currentTrack {
        | Some(track) =>
          switch (track.metadata.artist, track.metadata.album) {
          | (Some(trackArtist), _) if title === "Artists" => value === trackArtist
          | (_, Some(trackAlbum)) if title === "Albums" => value === trackAlbum
          | _ => false
          }
        | None => false
        }

        <div
          key={value}
          className={MillerColumnsViewStyles.columnItem(~isSelected, ~isCurrentTrackRelated)}
          onClick={_ => onSelect(value)}>
          {React.string(displayName)}
        </div>
      })
      ->React.array}
    </div>
  </div>
}
