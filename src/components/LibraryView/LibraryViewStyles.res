open CssJs

let container = style([
  padding(Spacing.containerPadding),
  height(#auto),
  overflowY(#visible),
  backgroundColor(Color.backgroundColor),
])

let header = style([
  display(#flex),
  justifyContent(#spaceBetween),
  alignItems(#center),
  marginBottom(Spacing.sectionMargin),
])

let artistGroup = style([
  marginBottom(px(32)),
])

let artistHeader = style([
  fontSize(px(20)),
  fontWeight(#bold),
  marginBottom(px(16)),
  color(Color.text),
])

let albumGroup = style([
  marginBottom(px(24)),
  marginLeft(px(16)),
])
