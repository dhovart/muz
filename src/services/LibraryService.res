let getLibraryPath = (): Promise.t<string> => {
  Tauri.invoke("get_library_path", ())
}

let setLibraryPath = (path: string): Promise.t<unit> => {
  Tauri.invoke("set_library_path", {"path": path})
}

let rescanLibrary = (): Promise.t<unit> => {
  Tauri.invoke("rescan_library", ())
}