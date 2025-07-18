open Mui

@react.component
let make = (
  ~tracks: array<Track.t>,
  ~currentTrack: option<Track.t>,
  ~onTrackSelect: option<Track.t => unit>=?,
  ~context: string="default",
) => {
  let handleTrackClick = (track: Track.t) => {
    switch onTrackSelect {
    | Some(callback) => callback(track)
    | None => ()
    }
  }

  let renderTrack = (track: Track.t, _index: int) => {
    let isCurrentTrack = switch currentTrack {
    | Some(current) => current.id == track.id
    | None => false
    }

    <div
      key={context ++ "_" ++ track.id}
      className={isCurrentTrack ? TrackListStyles.currentTrackRow : TrackListStyles.trackRow}
      onClick={_ => handleTrackClick(track)}>
      <div className=TrackListStyles.trackTitle> {track->Track.displayTitle->React.string} </div>
      {isCurrentTrack
        ? <div className=TrackListStyles.playingIndicator> {React.string("[Playing]")} </div>
        : React.null}
    </div>
  }

  <StyledEngineProvider injectFirst=true>
    <div className=TrackListStyles.container>
      {tracks->Array.mapWithIndex(renderTrack)->React.array}
    </div>
  </StyledEngineProvider>
}
