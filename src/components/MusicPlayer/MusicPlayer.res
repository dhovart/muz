module SkipPrevious = {
  @react.component @module("@mui/icons-material/SkipPrevious")
  external make: unit => React.element = "default"
}

module SkipNext = {
  @react.component @module("@mui/icons-material/SkipNext")
  external make: unit => React.element = "default"
}

module PlayArrow = {
  @react.component @module("@mui/icons-material/PlayArrow")
  external make: unit => React.element = "default"
}
module Pause = {
  @react.component @module("@mui/icons-material/Pause")
  external make: unit => React.element = "default"
}

open Mui
open State
open Command
open UseProgressData

@react.component
let make = () => {
  let player = PlayerContext.usePlayer()
  let (actualPosition, _framesPlayed) = useProgressData()
  let (isDragging, setIsDragging) = React.useState(() => false)
  let (dragPosition, setDragPosition) = React.useState(() => 0.0)
  let hasQueue = React.useMemo(() => player.queue->Array.length > 0, [player.queue])

  let displayPosition = if isDragging {
    dragPosition
  } else {
    actualPosition
  }

  let invokePlayerCommand = async command => {
    try {
      let result = await PlaybackService.controlPlayback(command)
      switch result {
      | Playing => player.dispatch(SetState(State.Playing))
      | Paused => player.dispatch(SetState(State.Paused))
      | Stopped => player.dispatch(SetState(State.Stopped))
      }
      Js.Console.log2("Player command invoked successfully", command)
    } catch {
    | Exn.Error(error) => Js.Console.error3("Error invoking player command", command, error)
    }
  }

  React.useEffect(() => {
    invokePlayerCommand(SetVolume(player.volume))->ignore
    None
  }, [player.volume])

  let handlePlayPause = React.useCallback(() => {
    switch player.state {
    | State.Playing => {
        player.dispatch(SetState(State.Paused))
        invokePlayerCommand(Command.Pause)->ignore
      }
    | State.Stopped
    | State.Paused => {
        player.dispatch(SetState(State.Playing))
        invokePlayerCommand(Command.Play)->ignore
      }
    }
  }, [player.state])

  let handleSeek = React.useCallback((value, currentTrack) => {
    switch currentTrack {
    | Some(track) =>
      let seekPositionMs = Track.getSeekPositionMs(track, value)
      invokePlayerCommand(Command.Seek(seekPositionMs))->ignore
    | None => Js.log("No current track for seeking")
    }
  }, [player.dispatch])

  let handlePrev = React.useCallback(() => {
    invokePlayerCommand(Command.Previous)->ignore
  }, [])

  let handleNext = React.useCallback(() => {
    invokePlayerCommand(Command.Next)->ignore
  }, [])

  <StyledEngineProvider injectFirst=true>
    <Grid
      className={MusicPlayerStyles.container}
      justifyContent=Center
      alignItems=Center
      container=true
      direction=Column>
      <div>
        {switch player.currentTrack {
        | Some(track) =>
          <>
            <Typography variant={H6}> {track->Track.displayTitle->React.string} </Typography>
            <Typography variant={Subtitle1}>
              {track->Track.displayArtist->React.string}
            </Typography>
          </>

        | None => <Typography variant={H6}> {React.string("No track selected")} </Typography>
        }}
      </div>
      <Slider
        className={MusicPlayerStyles.track}
        value=displayPosition
        step=Number(0.001)
        max=1.0
        onChange={(_, value, _) => {
          setIsDragging(_ => true)
          setDragPosition(_ => value)
        }}
        onChangeCommitted={(_, value) => {
          setIsDragging(_ => false)
          handleSeek(value, player.currentTrack)->ignore
        }}
      />
      <div>
        <IconButton onClick={_ => handlePrev()->ignore} disabled={!player.hasHistory}>
          <SkipPrevious />
        </IconButton>
        <Fab
          className={MusicPlayerStyles.playButton}
          color={Primary}
          onClick={_ => handlePlayPause()->ignore}>
          {player.state == State.Playing ? <Pause /> : <PlayArrow />}
        </Fab>
        <IconButton onClick={_ => handleNext()->ignore} disabled={!hasQueue}>
          <SkipNext />
        </IconButton>
      </div>
    </Grid>
  </StyledEngineProvider>
}
