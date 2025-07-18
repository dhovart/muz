open Command
open State

let getLibraryTracks = (): Promise.t<array<Track.t>> => {
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

type progressEvent = {positionPercent: int}

let subscribeToProgress = (onProgress: progressEvent => unit): Promise.t<unit> => {
  let channel: Tauri.channelType<progressEvent> = Tauri.channel()
  channel.onmessage = onProgress
  Tauri.invoke("subscribe_to_progress", {"onProgress": channel})
}
