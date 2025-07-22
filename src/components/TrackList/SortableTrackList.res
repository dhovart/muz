open Mui
open DndProvider

@react.component
let make = (
  ~tracks: array<Track.t>,
  ~currentTrack: option<Track.t>,
  ~onTrackSelect: Track.t => unit,
  ~context: string="queue",
) => {
  let renderTrack = (track: Track.t, index: int) => {
    let isCurrentTrack = switch currentTrack {
    | Some(current) => current.id == track.id
    | None => false
    }

    let handleTrackClick = _event => {
      onTrackSelect(track)
    }

    <QueueDnd.DraggableItem id={track.id} key={track.id} containerId={QueueContainer.id()} index>
      {#Children(
        <div
          className={isCurrentTrack
            ? TrackListStyles.queueCurrentTrackRow
            : TrackListStyles.queueTrackRow}
          onClick={handleTrackClick}>
          <div className=TrackListStyles.trackTitle>
            {track->Track.displayTitle->React.string}
          </div>
          {isCurrentTrack
            ? <div className=TrackListStyles.playingIndicator> {React.string("[Playing]")} </div>
            : React.null}
        </div>,
      )}
    </QueueDnd.DraggableItem>
  }

  <StyledEngineProvider injectFirst=true>
    <QueueDnd.DroppableContainer id={QueueContainer.id()} axis=Y>
      <div className=TrackListStyles.container>
        {tracks->Array.mapWithIndex(renderTrack)->React.array}
      </div>
    </QueueDnd.DroppableContainer>
  </StyledEngineProvider>
}
