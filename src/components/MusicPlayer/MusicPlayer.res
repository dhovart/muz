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

type historyPayload = {hasHistory: bool}

@module("@tauri-apps/api/core") @react.component
let make = () => {
  let albumArtUrl = "http://picsum.photos/1200/1200"

  let (state, setState) = React.useState(() => State.Paused)
  let (volume, _setVolume) = React.useState(() => 0.5)
  let (position, setPosition) = React.useState(() => 0.0)
  let (hasHistory, setHasHistory) = React.useState(() => false)
  let playerState = PlayerContext.usePlayer()

  let invokePlayerCommand = async command => {
    try {
      let _ret = await TrackService.controlPlayback(command)
      // Handle state updates based on command
      switch command {
      | Command.Play => setState(_ => State.Playing)
      | Command.Pause => setState(_ => State.Paused)
      | _ => ()
      }
      Js.Console.log2("Player command invoked successfully", command)
    } catch {
    | Exn.Error(error) => Js.Console.error3("Error invoking player command", command, error)
    }
  }

  React.useEffect(() => {
    invokePlayerCommand(SetVolume(volume))->ignore
    None
  }, [volume])

  React.useEffect(() => {
    let subscribeToProgress = async () => {
      try {
        await TrackService.subscribeToProgress(message => {
          setPosition(_ => message.positionPercent->Js.Int.toFloat /. 100.0)
        })
      } catch {
      | Exn.Error(error) => Js.Console.error2("Error subscribing to progress updates", error)
      }
    }

    let unlisten = ref(None)
    let listenToHistory = async () => {
      try {
        unlisten :=
          Some(
            await Tauri.listenToEvent("history-update", (payload: historyPayload) => {
              Js.Console.log2("History update received", payload)
              setHasHistory(_ => payload.hasHistory)
            }),
          )
      } catch {
      | Exn.Error(error) => Js.Console.error2("Error listening to history updates", error)
      }
    }

    subscribeToProgress()->ignore
    listenToHistory()->ignore

    Some(
      () => {
        if unlisten.contents != None {
          switch unlisten.contents {
          | Some(unlistenFn) => unlistenFn()->ignore
          | _ => ()
          }
        }
      },
    )
  }, [])

  let handlePlayPause = React.useCallback(() => {
    switch state {
    | State.Playing => {
        setState(_ => State.Paused)
        invokePlayerCommand(Command.Pause)->ignore
      }
    | State.Stopped
    | State.Paused => {
        setState(_ => State.Playing)
        invokePlayerCommand(Command.Play)->ignore
      }
    }
  }, [state])

  let handleSeek = React.useCallback(value => {
    setPosition(_ => value)
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
            switch playerState.currentTrack {
            | Some(track) => track->Track.displayTitle
            | None => "No track selected"
            },
          )}
        </Typography>
        <Typography variant={Subtitle1}> {React.string("Unknown Artist")} </Typography>
      </div>
      <Slider
        className={MusicPlayerStyles.track}
        value=position
        max=1.0
        onChange={(_, value, _) => handleSeek(value)->ignore}
      />
      <div>
        <IconButton onClick={_ => handlePrev()->ignore} disabled={!hasHistory}>
          <SkipPrevious />
        </IconButton>
        <Fab
          className={MusicPlayerStyles.playButton}
          color={Primary}
          onClick={_ => handlePlayPause()->ignore}>
          {state == State.Playing ? <Pause /> : <PlayArrow />}
        </Fab>
        <IconButton onClick={_ => handleNext()->ignore}>
          <SkipNext />
        </IconButton>
      </div>
    </Grid>
  </StyledEngineProvider>
}
