open CssJs
open CssJs.Types.Time

let container = style([display(#flex), flexDirection(#column), gap(px(1))])

let trackRow = style([
  display(#grid),
  unsafe("gridTemplateColumns", "1fr 40px"),
  gap(rem(1.)),
  padding2(~v=rem(1.), ~h=rem(1.)),
  backgroundColor(hex("f5f5f5")),
  borderRadius(rem(0.25)),
  cursor(#pointer),
  alignItems(#center),
  transition(~duration=ms(200.), ~timingFunction=#ease, "background-color"),
  hover([backgroundColor(hex("e0e0e0"))]),
])

let currentTrackRow = style([
  display(#grid),
  unsafe("gridTemplateColumns", "1fr 40px"),
  gap(rem(1.)),
  padding2(~v=rem(1.), ~h=rem(1.)),
  backgroundColor(hex("4b4d7f")),
  color(hex("ffffff")),
  borderRadius(rem(0.25)),
  cursor(#pointer),
  alignItems(#center),
  hover([backgroundColor(hex("3a3c66"))]),
])

let trackIndex = style([fontSize(rem(1.)), fontWeight(#medium), textAlign(#center)])

let trackTitle = style([
  fontSize(rem(1.)),
  fontWeight(#medium),
  overflow(#hidden),
  textOverflow(#ellipsis),
  whiteSpace(#nowrap),
])

let playingIndicator = style([display(#flex), justifyContent(#center), alignItems(#center)])
