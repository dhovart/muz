open CssJs

let container = style([
  maxWidth(rem(100.)),
  margin(auto),
  padding(Spacing.sectionMargin),
  minHeight(vh(100.)),
  display(flexBox),
  flexDirection(column),
  justifyContent(center),
  alignItems(center),
  position(#relative),
])

let playerContainer = style([
  width(#percent(100.)),
  padding2(~v=Spacing.md, ~h=Spacing.containerPadding),
  backgroundColor(Color.surfaceVariant),
  borderTop(Spacing.borderThin, #solid, Color.outline),
  display(flexBox),
  flexDirection(row),
  alignItems(center),
  gap(Spacing.lg),
  position(#fixed),
  bottom(zero),
  left(zero),
  zIndex(1),
])

let trackInfoSection = style([
  display(flexBox),
  flexDirection(row),
  alignItems(center),
  gap(Spacing.md),
  width(rem(18.75)),
  flexShrink(0.),
])

let albumArt = style([
  width(rem(3.75)),
  height(rem(3.75)),
  borderRadius(Spacing.xs),
  overflow(hidden),
  flexShrink(0.),
  selector("& img", [width(#percent(100.)), height(#percent(100.)), objectFit(cover)]),
])

let trackDetails = style([display(flexBox), flexDirection(column), minWidth(zero), flexGrow(1.)])

let controlsSection = style([
  display(flexBox),
  flexDirection(column),
  alignItems(center),
  gap(Spacing.sm),
  flexGrow(1.),
])
let track = style([
  selector("& .MuiSlider-thumb", [transition("none")]),
  selector("& .MuiSlider-track", [transition("none"), height(rem(0.375))]),
])
let playButton = style([
  marginLeft(Spacing.md),
  marginRight(Spacing.md),
  color(Color.textOnPrimary),
  borderRadius(rem(3.)),
  padding(Spacing.buttonPadding),
  active([transform(scale(1.05, 1.05))]),
])

let trackTitle = style([
  fontSize(rem(1.5)),
  fontWeight(#bold),
  color(Color.text),
  marginBottom(Spacing.textMedium),
  unsafe(
    "background",
    "linear-gradient(90deg, " ++
    Color.hexString(Color.primary) ++
    " 0%, " ++
    Color.hexString(Color.secondary) ++ " 100%)",
  ),
  unsafe("backgroundClip", "text"),
  unsafe("WebkitBackgroundClip", "text"),
  unsafe("WebkitTextFillColor", "transparent"),
])

let trackArtist = style([fontSize(rem(1.125)), color(Color.textSecondary), fontWeight(#medium)])

let trackInfo = style([
  display(flexBox),
  flexDirection(column),
  justifyContent(center),
  minWidth(rem(12.5)), // ~200px
  maxWidth(rem(18.75)), // ~300px
  marginRight(Spacing.lg),
])

let title = style([
  fontSize(rem(0.875)),
  fontWeight(#medium),
  color(Color.text),
  whiteSpace(nowrap),
  overflow(hidden),
  textOverflow(ellipsis),
  marginBottom(Spacing.textSmall),
])

let artist = style([
  fontSize(rem(0.75)),
  color(Color.textSecondary),
  whiteSpace(nowrap),
  overflow(hidden),
  textOverflow(ellipsis),
])

let album = style([
  fontSize(rem(0.75)),
  color(Color.textSecondary),
  whiteSpace(nowrap),
  overflow(hidden),
  textOverflow(ellipsis),
  fontStyle(italic),
])

let controls = style([display(flexBox), alignItems(center), gap(Spacing.elementGap)])

let trackSlider = style([
  width(#percent(100.)),
  selector("& .MuiSlider-thumb", [transition("none")]),
  selector("& .MuiSlider-track", [transition("none"), height(Spacing.xs)]),
  selector("& .MuiSlider-rail", [height(Spacing.xs)]),
])

let playerPlayButton = style([
  width(Spacing.xxl), // 48px
  height(Spacing.xxl),
  minWidth(Spacing.xxl),
  color(Color.textOnPrimary),
  padding(Spacing.sm),
])

let iconButton = style([
  width(Spacing.xl), // 32px
  height(Spacing.xl),
  minWidth(Spacing.xl),
  padding(Spacing.xs),
])
