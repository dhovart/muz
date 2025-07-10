module SkipPrevious = {
  @react.component @module("@mui/icons-material/SkipPrevious")
  external make: unit => React.element = "default"
}

module SkipNext = {
  @react.component @module("@mui/icons-material/SkipNext")
  external make: unit => React.element = "default"
}

module PlayArrow = {
  @react.component @module("@mui/icons-material/PlayArrow")
  external make: unit => React.element = "default"
}

module Pause = {
  @react.component @module("@mui/icons-material/Pause")
  external make: unit => React.element = "default"
}

open Mui

@react.component
let make = (
  ~albumArtUrl: string="http://picsum.photos/1200/1200",
  ~title: string="Unknown Song",
  ~artist: string="Unknown Artist",
  ~position: float=0.0,
  ~duration: float=0.0,
  ~isPlaying: bool=false,
  ~onPlayPause=unit => unit,
  ~onNext=unit => unit,
  ~onPrev=unit => unit,
) => {
  <Grid
    className={Styles.container}
    justifyContent=Center
    alignItems=Center
    container=true
    direction=Column>
    <img className={Styles.art} src=albumArtUrl alt="Album Art" />
    <div>
      <Typography variant={H6}> {React.string(title)} </Typography>
      <Typography variant={Subtitle1}> {React.string(artist)} </Typography>
    </div>
    <Slider className={Styles.track} value=position max=duration />
    <div>
      <IconButton onClick={_ => onPrev()}>
        <SkipPrevious />
      </IconButton>
      <Fab className={Styles.playButton} color={Primary} onClick={_ => onPlayPause()}>
        {isPlaying ? <Pause /> : <PlayArrow />}
      </Fab>
      <IconButton onClick={_ => onNext()}>
        <SkipNext />
      </IconButton>
    </div>
  </Grid>
}
