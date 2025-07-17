open CssJs
open CssJs.Types.Time

let container = style([padding(px(16)), height(vh(100.0)), overflowY(#auto)])

let header = style([
  display(#flex),
  justifyContent(#spaceBetween),
  alignItems(#center),
  marginBottom(px(16)),
])
