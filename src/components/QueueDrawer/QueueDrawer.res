open Mui
open PlayerContext
open QueueDrawerStyles
open Motion

module Close = {
  @react.component @module("@mui/icons-material/Close")
  external make: (~className: string=?) => React.element = "default"
}

@react.component
let make = (~isOpen: bool, ~onClose: unit => unit, ~onTrackSelect: option<Track.t => unit>=?) => {
  let player = PlayerContext.usePlayer()

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
                  ? {
                      player.queue
                      ->Array.map(track => {
                        let isCurrentTrack = switch player.currentTrack {
                        | Some(current) => current.id == track.id
                        | None => false
                        }

                        let initial = {"opacity": 0, "y": 20}
                        let animate = {"opacity": 1, "y": 0}
                        let exit = {"opacity": 0, "y": -20}
                        let transition = {"type": "spring", "damping": 20, "stiffness": 300}
                        let whileHover = {"scale": 1.02}
                        let whileTap = {"scale": 0.98}

                        let handleTrackClick = (track: Track.t) => {
                          switch onTrackSelect {
                          | Some(callback) => callback(track)
                          | None => ()
                          }
                        }

                        <MotionDiv
                          key={"queue_" ++ track.id}
                          layout=true
                          layoutId={"queue_" ++ track.id}
                          initial
                          animate
                          exit
                          transition
                          whileHover
                          whileTap
                          className={isCurrentTrack
                            ? TrackListStyles.currentTrackRow
                            : TrackListStyles.trackRow}
                          onClick={_ => handleTrackClick(track)}>
                          <div className=TrackListStyles.trackTitle>
                            {track->Track.displayTitle->React.string}
                          </div>
                        </MotionDiv>
                      })
                      ->React.array
                    }
                  : React.null}
              </LayoutGroup>
            </div>}
      </div>
    </Mui.Drawer>
  </StyledEngineProvider>
}
