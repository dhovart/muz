@react.component
let make = (
  ~title: string,
  ~tracks: array<Track.t>,
  ~artist: string,
  ~album: string,
  ~currentTrack: option<Track.t>,
) => {
  let player = PlayerContext.usePlayer()

  let playTrack = (track: Track.t) => {
    let maybeAlbum = album === "" ? None : Some(album)
    let maybeArtist = artist === "" ? None : Some(artist)
    PlaybackService.playFromLibrary(track.id, ~album=maybeAlbum, ~artist=maybeArtist, ())
    ->Promise.then(state => {
      player.dispatch(SetState(state))
      Promise.resolve()
    })
    ->ignore
  }

  <div className={MillerColumnsViewStyles.column}>
    <div className={MillerColumnsViewStyles.columnHeader}> {React.string(title)} </div>
    <div className={MillerColumnsViewStyles.columnContent}>
      {tracks
      ->Belt.Array.map(track => {
        let isCurrentTrack = switch currentTrack {
        | Some(currentTrack) => currentTrack.id === track.id
        | None => false
        }

        <div
          key={track.id}
          className={MillerColumnsViewStyles.trackItem(~isCurrentTrack)}
          onClick={_ => playTrack(track)}>
          <div className={MillerColumnsViewStyles.trackTitle}>
            {Track.displayTitle(track)->React.string}
          </div>
        </div>
      })
      ->React.array}
    </div>
  </div>
}
