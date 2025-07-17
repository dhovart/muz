open Command

let getLibraryTracks = (): Promise.t<array<Track.t>> => {
  Tauri.invoke("get_library_tracks", ())
}

let controlPlayback = (command: Command.t): Promise.t<unit> => {
  Tauri.invoke("control_playback", Command.toJsonPayload(command))
}

type progressEvent = {positionPercent: int}

let subscribeToProgress = (onProgress: progressEvent => unit): Promise.t<unit> => {
  let channel: Tauri.channelType<progressEvent> = Tauri.channel()
  channel.onmessage = onProgress
  Tauri.invoke("subscribe_to_progress", {"onProgress": channel})
}
