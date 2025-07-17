open CssJs

let drawerContent = style([width(px(400)), padding(px(16)), height(vh(100.0)), overflowY(#auto)])

let header = style([
  display(#flex),
  justifyContent(#spaceBetween),
  alignItems(#center),
  marginBottom(px(16)),
])

let emptyState = style([
  display(#flex),
  flexDirection(#column),
  alignItems(#center),
  justifyContent(#center),
  height(px(200)),
  opacity(0.6),
  textAlign(#center),
])
