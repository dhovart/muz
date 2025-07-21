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
    | State.Playing => invokePlayerCommand(Command.Pause)->ignore
    | State.Stopped
    | State.Paused =>
      invokePlayerCommand(Command.Play)->ignore
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
    <div className={MusicPlayerStyles.playerContainer}>
      <div className={MusicPlayerStyles.trackInfoSection}>
        <div className={MusicPlayerStyles.albumArt}>
          <img src="https://picsum.photos/200/200" alt="" />
        </div>
        <div className={MusicPlayerStyles.trackDetails}>
          {switch player.currentTrack {
          | Some(track) =>
            <>
              <div className={MusicPlayerStyles.title}>
                {track->Track.displayTitle->React.string}
              </div>
              <div className={MusicPlayerStyles.artist}>
                {track->Track.displayArtist->React.string}
              </div>
              <div className={MusicPlayerStyles.album}>
                {track->Track.displayAlbum->React.string}
              </div>
            </>
          | None =>
            <>
              <div className={MusicPlayerStyles.title}> {React.string("No track selected")} </div>
              <div className={MusicPlayerStyles.album}> {React.string("No album")} </div>
            </>
          }}
        </div>
      </div>
      <div className={MusicPlayerStyles.controlsSection}>
        <div className={MusicPlayerStyles.controls}>
          <IconButton
            className={MusicPlayerStyles.iconButton}
            onClick={_ => handlePrev()->ignore}
            disabled={!player.hasHistory}>
            <SkipPrevious />
          </IconButton>
          <Fab
            className={MusicPlayerStyles.playerPlayButton}
            color={Primary}
            onClick={_ => handlePlayPause()->ignore}>
            {player.state == State.Playing ? <Pause /> : <PlayArrow />}
          </Fab>
          <IconButton
            className={MusicPlayerStyles.iconButton}
            onClick={_ => handleNext()->ignore}
            disabled={!hasQueue}>
            <SkipNext />
          </IconButton>
        </div>
        <Slider
          className={MusicPlayerStyles.trackSlider}
          value=displayPosition
          step=Number(0.001)
          max=1.0
          disabled={Belt.Option.isNone(player.currentTrack)}
          onChange={(_, value, _) => {
            setIsDragging(_ => true)
            setDragPosition(_ => value)
          }}
          onChangeCommitted={(_, value) => {
            setIsDragging(_ => false)
            handleSeek(value, player.currentTrack)->ignore
          }}
        />
      </div>
    </div>
  </StyledEngineProvider>
}
