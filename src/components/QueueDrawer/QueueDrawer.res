open Mui
open PlayerContext
open QueueDrawerStyles
open Motion

module Close = {
  @react.component @module("@mui/icons-material/Close")
  external make: (~className: string=?) => React.element = "default"
}

@react.component
let make = (~isOpen: bool, ~onClose: unit => unit) => {
  let player = PlayerContext.usePlayer()

  let handleTrackSelect = React.useCallback0((track: Track.t) => {
    PlaybackService.selectTrackFromQueue(track.id)
    ->Promise.then(state => {
      player.dispatch(SetState(state))
      Promise.resolve()
    })
    ->ignore
  })

  let handleDndReorder = React.useCallback1(
    (result: Dnd.result<DndProvider.TrackItem.t, DndProvider.QueueContainer.t>) => {
      switch result {
      | Some(SameContainer(draggedTrackId, placement)) => {
          let tracks = player.queue
          let oldIndex = tracks->Array.findIndex(track => track.id === draggedTrackId)

          if oldIndex !== -1 {
            let newIndex = switch placement {
            | Before(beforeTrackId) => {
                let beforeIndex = tracks->Array.findIndex(track => track.id === beforeTrackId)
                if beforeIndex !== -1 {
                  if oldIndex < beforeIndex {
                    beforeIndex - 1
                  } else {
                    beforeIndex
                  }
                } else {
                  oldIndex
                }
              }
            | Last => Array.length(tracks) - 1
            }

            if oldIndex !== newIndex {
              PlaybackService.reorderQueue(oldIndex, newIndex)->ignore
            }
          }
        }
      | _ => ()
      }
    },
    [player.queue],
  )

  let renderCurrentTrack = (track: Track.t) => {
    let initial = {"opacity": 0, "y": -20}
    let animate = {"opacity": 1, "y": 0}
    let exit = {"opacity": 0, "y": -20}
    let transition = {"type": "spring", "damping": 25, "stiffness": 350}

    <MotionDiv
      key={"queue_current_" ++ track.id}
      layout=true
      layoutId={"queue_current_" ++ track.id}
      initial
      animate
      exit
      transition
      className=TrackListStyles.currentTrackInQueue>
      <div>
        <div className=TrackListStyles.nowPlayingLabel> {React.string("Now Playing")} </div>
        <div className=TrackListStyles.trackTitle> {track->Track.displayTitle->React.string} </div>
      </div>
      <div className=TrackListStyles.playingIndicator> {React.string("♪")} </div>
    </MotionDiv>
  }

  <StyledEngineProvider injectFirst=true>
    <Mui.Drawer anchor=Right open_=isOpen onClose={(_, _) => onClose()}>
      <div className=drawerContent>
        <div className=header>
          <h2> {"Queue"->React.string} </h2>
          <Mui.IconButton onClick={_ => onClose()}>
            <Close />
          </Mui.IconButton>
        </div>
        {player.queue->Array.length == 0 && player.currentTrack == None
          ? <div className=emptyState>
              <p> {"No tracks in queue"->React.string} </p>
            </div>
          : <div className=TrackListStyles.container>
              <LayoutGroup>
                {switch player.currentTrack {
                | Some(current) => renderCurrentTrack(current)
                | None => React.null
                }}
                {player.queue->Array.length > 0
                  ? <DndProvider onReorder=handleDndReorder>
                      <SortableTrackList
                        tracks=player.queue
                        currentTrack=player.currentTrack
                        onTrackSelect={track => handleTrackSelect(track)}
                        context="queue"
                      />
                    </DndProvider>
                  : React.null}
              </LayoutGroup>
            </div>}
      </div>
    </Mui.Drawer>
  </StyledEngineProvider>
}
