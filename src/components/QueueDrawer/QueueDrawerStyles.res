open CssJs

let drawerContent = style([
  width(rem(25.)),
  padding(Spacing.containerPadding),
  height(vh(100.0)),
  overflowY(#auto),
  backgroundColor(Color.surface),
])

let header = style([
  display(#flex),
  justifyContent(#spaceBetween),
  alignItems(#center),
  marginBottom(Spacing.sectionMargin),
])

let emptyState = style([
  display(#flex),
  flexDirection(#column),
  alignItems(#center),
  justifyContent(#center),
  height(rem(12.5)),
  opacity(0.6),
  textAlign(#center),
  color(Color.textSecondary),
])
