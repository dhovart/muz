module Settings = {
  @react.component @module("@mui/icons-material/Settings")
  external make: (~className: string=?) => React.element = "default"
}

module MusicNote = {
  @react.component @module("@mui/icons-material/MusicNote")
  external make: (~className: string=?) => React.element = "default"
}

open Mui

@react.component
let make = (~currentPage: Route.t, ~onPageChange: Route.t => unit) => {
  <div className={MenuStyles.container}>
    <Box className={MenuStyles.menuBox}>
      <IconButton 
        onClick={_ => onPageChange(Route.MusicPlayer)} 
        color={currentPage == Route.MusicPlayer ? Primary : Default}>
        <MusicNote />
      </IconButton>
      <IconButton 
        onClick={_ => onPageChange(Route.Settings)} 
        color={currentPage == Route.Settings ? Primary : Default}>
        <Settings />
      </IconButton>
    </Box>
  </div>
}