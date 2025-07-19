let getLibraryPath = (): Promise.t<string> => {
  Tauri.invoke("get_library_path", ())
}

let setLibraryPath = (path: string) => {
  Tauri.invoke("set_library_path", {"path": path})
}

let rescanLibrary = (~onRescan: option<unit => unit>) => {
  switch onRescan {
  | Some(fn) => fn()
  | None => ()
  }

  Tauri.invoke("rescan_library", ())
}

let getAlbumsByArtist = (): Promise.t<Js.Dict.t<Js.Dict.t<array<Track.t>>>> => {
  Tauri.invoke("get_albums_by_artist", ())
}
