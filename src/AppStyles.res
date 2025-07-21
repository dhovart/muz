open CssJs

let appContainer = style([
  height(vh(100.)),
  display(flexBox),
  flexDirection(column),
])

let contentArea = style([
  flexGrow(1.0),
  overflowY(#auto),
  overflowX(hidden),
  paddingBottom(rem(5.)), // 80px for the player
])