type playerAction =
  | SetCurrentTrack(option<Track.t>)
  | SetQueue(array<Track.t>)
  | SetHasHistory(bool)
  | SetVolume(float)
  | SetState(State.t)

type playerState = {
  currentTrack: option<Track.t>,
  queue: array<Track.t>,
  hasHistory: bool,
  volume: float,
  state: State.t,
  cleanupListeners: unit => unit,
  dispatch: playerAction => unit,
}

let initialPlayerState: playerState = {
  currentTrack: None,
  queue: [],
  hasHistory: false,
  volume: 0.5,
  state: State.Stopped,
  cleanupListeners: () => (),
  dispatch: _ => (),
}

let playerContext = React.createContext(initialPlayerState)

module Provider = {
  let make = React.Context.provider(playerContext)
}

module PlayerProvider = {
  @react.component
  let make = (~children, ~state, ~dispatch) => {
    let cleanupRef = React.useRef(() => ())
    let cleanupListeners = () => cleanupRef.current()

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
      ...state,
      dispatch,
      cleanupListeners,
    }

    <Provider value={contextValue}> children </Provider>
  }
}

let usePlayer = () => {
  React.useContext(playerContext)
}

let usePlayerReducer = () => {
  let playerReducer = (state: playerState, action: playerAction) => {
    switch action {
    | SetCurrentTrack(track) => {...state, currentTrack: track}
    | SetQueue(queue) => {...state, queue}
    | SetHasHistory(hasHistory) => {...state, hasHistory}
    | SetVolume(volume) => {...state, volume}
    | SetState(playerState) => {...state, state: playerState}
    }
  }

  React.useReducer(playerReducer, initialPlayerState)
}
