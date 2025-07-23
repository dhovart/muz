module Settings = {
  @react.component @module("@mui/icons-material/Settings")
  external make: (~className: string=?) => React.element = "default"
}

module MusicNote = {
  @react.component @module("@mui/icons-material/MusicNote")
  external make: (~className: string=?) => React.element = "default"
}

module QueueMusic = {
  @react.component @module("@mui/icons-material/QueueMusic")
  external make: (~className: string=?) => React.element = "default"
}

module Visualizer = {
  @react.component @module("@mui/icons-material/Fullscreen")
  external make: (~className: string=?) => React.element = "default"
}

module ViewColumn = {
  @react.component @module("@mui/icons-material/ViewColumn")
  external make: (~className: string=?) => React.element = "default"
}

open Mui

@react.component
let make = (
  ~currentPage: Route.t,
  ~onPageChange: Route.t => unit,
  ~onQueueToggle: unit => unit,
) => {
  <div className={MenuStyles.container}>
    <Box className={MenuStyles.menuBox}>
      <IconButton
        onClick={_ => onPageChange(Route.MillerColumns)}
        color={currentPage == Route.MillerColumns ? Primary : Default}>
        <ViewColumn />
      </IconButton>
      <IconButton onClick={_ => onQueueToggle()}>
        <QueueMusic />
      </IconButton>
      <IconButton
        onClick={_ => onPageChange(Route.Visualizer)}
        color={currentPage == Route.Visualizer ? Primary : Default}>
        <Visualizer />
      </IconButton>
      <IconButton
        onClick={_ => onPageChange(Route.Settings)}
        color={currentPage == Route.Settings ? Primary : Default}>
        <Settings />
      </IconButton>
    </Box>
  </div>
}
