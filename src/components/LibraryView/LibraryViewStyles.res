open CssJs

let container = style([
  padding(Spacing.containerPadding),
  height(vh(100.0)),
  overflowY(#auto),
  backgroundColor(Color.backgroundColor),
])

let header = style([
  display(#flex),
  justifyContent(#spaceBetween),
  alignItems(#center),
  marginBottom(Spacing.sectionMargin),
])
