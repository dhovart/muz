open CssJs
open CssJs.Types.Time

let container = style([padding(rem(1.)), height(vh(100.0)), overflowY(#auto)])

let header = style([
  display(#flex),
  justifyContent(#spaceBetween),
  alignItems(#center),
  marginBottom(rem(1.)),
])
