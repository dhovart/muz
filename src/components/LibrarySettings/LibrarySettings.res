module Folder = {
  @react.component @module("@mui/icons-material/Folder")
  external make: unit => React.element = "default"
}

module Settings = {
  @react.component @module("@mui/icons-material/Settings")
  external make: (~className: string=?) => React.element = "default"
}

open Mui

@react.component
let make = () => {
  let (libraryPath, setLibraryPath) = React.useState(() => "")
  let (isLoading, setIsLoading) = React.useState(() => false)
  let (error, setError) = React.useState(() => None)

  let loadLibraryPath = async () => {
    try {
      let path = await Tauri.invoke("get_library_path", ())
      setLibraryPath(_ => path)
    } catch {
    | Exn.Error(e) => {
        setError(_ => Some("Failed to load library path"))
        Js.Console.error2("Error loading library path", e)
      }
    }
  }

  let handleSelectFolder = async () => {
    try {
      setIsLoading(_ => true)
      let selectedPath = await Tauri.openFolderDialog()
      switch selectedPath->Nullable.toOption {
      | Some(path) => {
          let _ = await Tauri.invoke("set_library_path", {"path": path})
          setLibraryPath(_ => path)
          setError(_ => None)
        }
      | None => ()
      }
    } catch {
    | Exn.Error(e) => {
        setError(_ => Some("Failed to set library path"))
        Js.Console.error2("Error setting library path", e)
      }
    }
    setIsLoading(_ => false)
  }

  React.useEffect(() => {
    let _ = Tauri.invoke("rescan_library", ())->ignore
    None
  }, [libraryPath])

  React.useEffect(() => {
    loadLibraryPath()->ignore
    None
  }, [])

  <StyledEngineProvider injectFirst=true>
    <Card className={LibrarySettingsStyles.container}>
      <CardContent>
        <Typography variant={H6} gutterBottom=true className={LibrarySettingsStyles.settingsHeader}>
          <Settings className={LibrarySettingsStyles.settingsIcon} />
          {React.string(" Library Settings")}
        </Typography>
        <div className={LibrarySettingsStyles.pathContainer}>
          <Typography variant={Body2} color={TextSecondary}>
            {React.string("Current library path:")}
          </Typography>
          <Typography variant={Body1} className={LibrarySettingsStyles.currentPath}>
            {React.string(libraryPath)}
          </Typography>
        </div>
        {switch error {
        | Some(errorMsg) =>
          <Alert severity={Error} className={LibrarySettingsStyles.errorAlert}>
            {React.string(errorMsg)}
          </Alert>
        | None => React.null
        }}
        <Button
          variant={Contained}
          startIcon={<Folder />}
          onClick={_ => handleSelectFolder()->ignore}
          disabled={isLoading}
          className={LibrarySettingsStyles.selectButton}>
          {React.string(isLoading ? "Selecting..." : "Select Library Folder")}
        </Button>
      </CardContent>
    </Card>
  </StyledEngineProvider>
}
