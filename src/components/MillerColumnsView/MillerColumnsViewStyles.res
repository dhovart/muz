open CssJs
open Spacing

let container = style([
  display(#flex),
  flexDirection(#column),
  height(#percent(100.0)),
  width(#percent(100.0)),
  borderTop(borderThin, #solid, Color.outline),
])

let columnsContainer = style([
  display(#flex),
  flexDirection(#row),
  height(#percent(100.0)),
  overflowX(#auto),
  overflowY(#hidden),
])

let column = style([
  display(#flex),
  flexDirection(#column),
  flex(#num(1.0)),
  minWidth(rem(12.5)),
  borderRight(borderThin, #solid, Color.outline),
  backgroundColor(Color.surface),
])

let columnHeader = style([
  padding2(~v=md, ~h=containerPadding),
  backgroundColor(Color.surfaceVariant),
  borderBottom(borderThin, #solid, Color.outline),
  fontWeight(#num(600)),
  fontSize(rem(0.875)),
  color(Color.textSecondary),
  textTransform(#uppercase),
  letterSpacing(rem(0.0625)),
])

let columnContent = style([flex(#num(1.0)), overflow(#auto), minHeight(#zero)])

let columnItem = (~isSelected: bool, ~isCurrentTrackRelated: bool) =>
  style([
    padding2(~v=sm, ~h=md),
    borderBottom(borderThin, #solid, Color.outlineVariant),
    cursor(#pointer),
    fontSize(rem(0.875)),
    color(
      if isSelected {
        Color.textOnPrimary
      } else {
        Color.text
      },
    ),
    transition(~duration=#ms(200.), "all"),
    backgroundColor(
      if isSelected {
        Color.selected
      } else if isCurrentTrackRelated {
        Color.queueItem
      } else {
        #transparent
      },
    ),
    borderLeft(
      borderThick,
      #solid,
      if isSelected {
        Color.primary
      } else if isCurrentTrackRelated {
        Color.secondary
      } else {
        #transparent
      },
    ),
    hover([
      backgroundColor(
        if isSelected {
          Color.selectedHover
        } else {
          Color.hoverColor
        },
      ),
    ]),
  ])

let trackItem = (~isCurrentTrack: bool) =>
  style([
    display(#flex),
    alignItems(#center),
    padding2(~v=sm, ~h=md),
    borderBottom(borderThin, #solid, Color.outlineVariant),
    cursor(#pointer),
    fontSize(rem(0.875)),
    color(
      if isCurrentTrack {
        Color.textOnPrimary
      } else {
        Color.text
      },
    ),
    transition(~duration=#ms(200.), "all"),
    backgroundColor(
      if isCurrentTrack {
        Color.selected
      } else {
        #transparent
      },
    ),
    borderLeft(
      borderThick,
      #solid,
      if isCurrentTrack {
        Color.primary
      } else {
        #transparent
      },
    ),
    hover([
      backgroundColor(
        if isCurrentTrack {
          Color.selectedHover
        } else {
          Color.hoverColor
        },
      ),
    ]),
  ])

let trackNumber = style([
  minWidth(rem(1.875)),
  fontSize(rem(0.75)),
  color(Color.textSecondary),
  fontWeight(#num(500)),
])

let trackTitle = style([
  flex(#num(1.0)),
  whiteSpace(#nowrap),
  overflow(#hidden),
  textOverflow(#ellipsis),
])
