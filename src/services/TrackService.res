open Command
open State

let getLibraryTracks = (): Promise.t<Js.Dict.t<array<Track.t>>> => {
  Tauri.invoke("get_library_tracks", ())
}

let controlPlayback = async (command: Command.t): State.t => {
  let result = await Tauri.invoke("control_playback", Command.toJsonPayload(command))
  switch result {
  | "Playing" => State.Playing
  | "Paused" => State.Paused
  | _ => State.Stopped
  }
}

type progressEvent = {position: float, spectrumData: array<float>}

let subscribeToProgress = (onProgress: progressEvent => unit): Promise.t<unit> => {
  let channel: Tauri.channelType<progressEvent> = Tauri.channel()
  channel.onmessage = onProgress
  Tauri.invoke("subscribe_to_progress", {"onProgress": channel})
}

let selectTrackFromQueue = async (trackId: string): State.t => {
  let result = await Tauri.invoke("select_track_from_queue", {"trackId": trackId})
  switch result {
  | "Playing" => State.Playing
  | "Paused" => State.Paused
  | _ => State.Stopped
  }
}

let playFromLibrary = async (
  trackId: string,
  ~album: option<string>,
  ~artist: option<string>,
  (),
): State.t => {
  let payload = Js.Dict.empty()
  Js.Dict.set(payload, "trackId", trackId)
  switch album {
  | Some(albumName) => Js.Dict.set(payload, "album", albumName)
  | None => ()
  }
  switch artist {
  | Some(artistName) => Js.Dict.set(payload, "artist", artistName)
  | None => ()
  }

  let result = await Tauri.invoke("play_from_library", payload)
  switch result {
  | "Playing" => State.Playing
  | "Paused" => State.Paused
  | _ => State.Stopped
  }
}
