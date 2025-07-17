open Mui
open PlayerContext
open QueueDrawerStyles

module Close = {
  @react.component @module("@mui/icons-material/Close")
  external make: (~className: string=?) => React.element = "default"
}

@react.component
let make = (~isOpen: bool, ~onClose: unit => unit, ~onTrackSelect: option<Track.t => unit>=?) => {
  let playerState = PlayerContext.usePlayer()

  <StyledEngineProvider injectFirst=true>
    <Mui.Drawer anchor=Right open_=isOpen onClose={(_, _) => onClose()}>
      <div className=drawerContent>
        <div className=header>
          <h2> {"Queue"->React.string} </h2>
          <Mui.IconButton onClick={_ => onClose()}>
            <Close />
          </Mui.IconButton>
        </div>
        {playerState.queue->Array.length == 0
          ? <div className=emptyState>
              <p> {"No tracks in queue"->React.string} </p>
            </div>
          : <TrackList tracks=playerState.queue currentTrack=playerState.currentTrack />}
      </div>
    </Mui.Drawer>
  </StyledEngineProvider>
}
