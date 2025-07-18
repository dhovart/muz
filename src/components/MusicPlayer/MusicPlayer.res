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

open State
open Mui
open Command

type progressEvent = {positionPercent: int}
type progressSubscriptionPayload = {onProgress: Tauri.channelType<progressEvent>}

@react.component
let make = () => {
  let albumArtUrl = "http://picsum.photos/1200/1200"

  let player = PlayerContext.usePlayer()
  let hasQueue = React.useMemo(() => player.queue->Array.length > 0, [player.queue])

  let invokePlayerCommand = async command => {
    try {
      let result = await TrackService.controlPlayback(command)
      switch result {
      | Playing => player.setState(State.Playing)
      | Paused => player.setState(State.Paused)
      | Stopped => player.setState(State.Stopped)
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
        player.setState(State.Paused)
        invokePlayerCommand(Command.Pause)->ignore
      }
    | State.Stopped
    | State.Paused => {
        player.setState(State.Playing)
        invokePlayerCommand(Command.Play)->ignore
      }
    }
  }, [player.state])

  let handleSeek = React.useCallback(value => {
    invokePlayerCommand(Command.Seek(value))->ignore
  }, [])

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
      <img className={MusicPlayerStyles.art} src=albumArtUrl alt="Album Art" />
      <div>
        <Typography variant={H6}>
          {React.string(
            switch player.currentTrack {
            | Some(track) => track->Track.displayTitle
            | None => "No track selected"
            },
          )}
        </Typography>
        <Typography variant={Subtitle1}> {React.string("Unknown Artist")} </Typography>
      </div>
      <Slider
        className={MusicPlayerStyles.track}
        value=player.position
        max=1.0
        onChange={(_, value, _) => handleSeek(value)->ignore}
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
