type playerState = {
  currentTrack: option<Track.t>,
  queue: array<Track.t>,
}

type playerAction =
  | SetCurrentTrack(option<Track.t>)
  | SetQueue(array<Track.t>)

let playerContext = React.createContext({
  currentTrack: None,
  queue: [],
})

module Provider = {
  let make = React.Context.provider(playerContext)
}

module PlayerProvider = {
  @react.component
  let make = (~state: playerState, ~dispatch, ~children) => {
    React.useEffect(() => {
      let setupEventListeners = async () => {
        try {
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

          () => {
            unlistenTrackChanged()->ignore
            unlistenQueueChanged()->ignore
          }
        } catch {
        | Exn.Error(error) => {
            Js.Console.error2("Error setting up player event listeners", error)
            () => ()
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

    <Provider value={state}> children </Provider>
  }
}

let usePlayer = () => {
  React.useContext(playerContext)
}
