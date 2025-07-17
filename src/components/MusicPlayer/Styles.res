open CssJs
let container = style([maxWidth(rem(100.)), margin(auto), padding(rem(1.))])
let art = style([
  width(#percent(100.)),
  height(auto),
  unsafe("aspectRatio", "1 / 1"),
  objectFit(cover),
  marginBottom(rem(1.)),
  maxWidth(rem(20.)),
])
let track = style([
  width(pct(100.)),
  marginBottom(rem(1.)),
  maxWidth(rem(20.)),
  selector("& .MuiSlider-thumb", [transition("none")]),
  selector("& .MuiSlider-track", [transition("none")]),
])
let playButton = style([marginLeft(rem(1.)), marginRight(rem(1.))])
