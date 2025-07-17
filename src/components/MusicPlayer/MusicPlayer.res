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
  let title = "Unknown Song"
  let artist = "Unknown Artist"

  let (state, setState) = React.useState(() => State.Paused)
  let (volume, setVolume) = React.useState(() => 0.5)
  let (position, setPosition) = React.useState(() => 0.0)
  let (hasHistory, setHasHistory) = React.useState(() => false)

  let invokePlayerCommand = async command => {
    let payload = Command.toJsonPayload(command)

    try {
      let ret = await Tauri.invokeCommand("control_playback", payload)
      switch ret {
      | "Playing" => setState(_ => State.Playing)
      | "Paused" => setState(_ => State.Paused)
      | "Stopped" => setState(_ => State.Stopped)
      | _ => Js.Console.warn2("Unknown playback state received", ret)
      }
      Js.Console.log2("Player command invoked successfully", ret)
    } catch {
    | Exn.Error(error) => Js.Console.error3("Error invoking player command", payload, error)
    }
  }

  React.useEffect(() => {
    invokePlayerCommand(SetVolume(volume))->ignore
    None
  }, [volume])

  React.useEffect(() => {
    let subscribeToProgress = async () => {
      try {
        let onProgress: Tauri.channelType<progressEvent> = Tauri.channel()
        onProgress.onmessage = message => {
          setPosition(_ => message.positionPercent->Js.Int.toFloat /. 100.0)
        }
        await Tauri.invokeCommand("subscribe_to_progress", {onProgress: onProgress})
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
        <Typography variant={H6}> {React.string(title)} </Typography>
        <Typography variant={Subtitle1}> {React.string(artist)} </Typography>
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
