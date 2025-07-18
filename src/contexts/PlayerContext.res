open TrackService
open State

type playerState = {
  currentTrack: option<Track.t>,
  queue: array<Track.t>,
  hasHistory: bool,
  position: float,
  volume: float,
  state: State.t,
}

type playerAction =
  | SetCurrentTrack(option<Track.t>)
  | SetQueue(array<Track.t>)
  | SetHasHistory(bool)
  | SetPosition(float)
  | SetVolume(float)
  | SetState(State.t)

type playerContextType = {
  currentTrack: option<Track.t>,
  queue: array<Track.t>,
  hasHistory: bool,
  position: float,
  volume: float,
  state: State.t,
  setState: State.t => unit,
  cleanupListeners: unit => unit,
}

let playerContext = React.createContext({
  currentTrack: None,
  queue: [],
  hasHistory: false,
  position: 0.0,
  volume: 0.5,
  state: State.Stopped,
  setState: _ => (),
  cleanupListeners: () => (),
})

module Provider = {
  let make = React.Context.provider(playerContext)
}

module PlayerProvider = {
  @react.component
  let make = (~state: playerState, ~dispatch, ~children) => {
    let cleanupRef = React.useRef(() => ())

    React.useEffect(() => {
      let setupEventListeners = async () => {
        try {
          Js.Console.log("Setting up player event listeners")

          // Clean up any existing listeners first
          cleanupRef.current()

          // Listen for track changes
          let unlistenTrackChanged = await Tauri.listenToEvent("track-changed", (
            payload: {"track": Nullable.t<Track.t>},
          ) => {
            Js.Console.log2("Track changed event received", payload)
            let maybeTrack = payload["track"]->Nullable.toOption
            switch maybeTrack {
            | Some(track) => dispatch(SetCurrentTrack(Some(track)))
            | None => ()
            }
          })

          // Listen for queue changes
          let unlistenQueueChanged = await Tauri.listenToEvent("queue-changed", (
            payload: {"queue": array<Track.t>},
          ) => {
            Js.Console.log2("Queue changed event received", payload)
            dispatch(SetQueue(payload["queue"]))
          })

          // Listen for history changes
          let unlistenHistoryChanged = await Tauri.listenToEvent("history-update", (
            payload: {"hasHistory": bool},
          ) => {
            Js.Console.log2("History update event received", payload)
            dispatch(SetHasHistory(payload["hasHistory"]))
          })

          // Subscribe to progress updates
          let _ = await TrackService.subscribeToProgress(message => {
            let position = message.positionPercent->Js.Int.toFloat /. 100.0
            dispatch(SetPosition(position))
          })

          let cleanup = () => {
            unlistenTrackChanged()->ignore
            unlistenQueueChanged()->ignore
            unlistenHistoryChanged()->ignore
          }

          cleanupRef.current = cleanup
          cleanup
        } catch {
        | Exn.Error(error) => {
            Js.Console.error2("Error setting up player event listeners", error)
            () => cleanupRef.current()
          }
        }
      }

      setupEventListeners()
      ->Promise.then(cleanup => {
        Promise.resolve(Some(cleanup))
      })
      ->ignore

      None
    }, [])

    let contextValue = {
      currentTrack: state.currentTrack,
      queue: state.queue,
      hasHistory: state.hasHistory,
      position: state.position,
      volume: state.volume,
      state: state.state,
      setState: newState => dispatch(SetState(newState)),
      cleanupListeners: () => cleanupRef.current(),
    }

    <Provider value={contextValue}> children </Provider>
  }
}

let usePlayer = () => {
  React.useContext(playerContext)
}
