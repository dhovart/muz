open CssJs

let container = style([display(#flex), flexDirection(#column), gap(Spacing.elementGap)])

let trackRow = style([
  display(#grid),
  unsafe("gridTemplateColumns", "1fr 40px"),
  gap(Spacing.md),
  padding2(~v=Spacing.md, ~h=Spacing.md),
  backgroundColor(Color.queueItem),
  borderRadius(Spacing.sm),
  cursor(#pointer),
  alignItems(#center),
  hover([backgroundColor(Color.queueItemHover)]),
])

let currentTrackRow = style([
  display(#grid),
  unsafe("gridTemplateColumns", "1fr 40px"),
  gap(Spacing.md),
  padding2(~v=Spacing.md, ~h=Spacing.md),
  unsafe(
    "background",
    "linear-gradient(135deg, " ++
    Color.hexString(Color.primary) ++
    " 0%, " ++
    Color.hexString(Color.primaryDark) ++ " 100%)",
  ),
  color(Color.textOnPrimary),
  borderRadius(Spacing.md),
  cursor(#pointer),
  alignItems(#center),
  border(Spacing.borderMedium, #solid, Color.primaryLight),
  hover([
    unsafe(
      "background",
      "linear-gradient(135deg, " ++
      Color.hexString(Color.primaryDark) ++
      " 0%, " ++
      Color.hexString(Color.primary) ++ " 100%)",
    ),
  ]),
])

let trackIndex = style([fontSize(rem(1.)), fontWeight(#medium), textAlign(#center)])

let trackTitle = style([
  fontSize(rem(1.)),
  fontWeight(#medium),
  overflow(#hidden),
  textOverflow(#ellipsis),
  whiteSpace(#nowrap),
])

let playingIndicator = style([
  display(#flex),
  justifyContent(#center),
  alignItems(#center),
  fontSize(rem(1.2)),
])

let currentTrackInQueue = style([
  display(#grid),
  unsafe("gridTemplateColumns", "1fr 40px"),
  gap(Spacing.md),
  padding2(~v=Spacing.lg, ~h=Spacing.lg),
  unsafe(
    "background",
    "linear-gradient(135deg, " ++
    Color.hexString(Color.primary) ++
    " 0%, " ++
    Color.hexString(Color.primaryDark) ++ " 100%)",
  ),
  color(Color.textOnPrimary),
  borderRadius(Spacing.lg),
  cursor(#default),
  alignItems(#center),
  border(Spacing.borderThick, #solid, Color.primaryLight),
  position(#relative),
  marginBottom(Spacing.lg),
  transform(scale(1.02, 1.02)),
])

let nowPlayingLabel = style([
  color(Color.textOnPrimary),
  fontSize(rem(0.7)),
  fontWeight(#bold),
  textTransform(#uppercase),
  letterSpacing(Spacing.xs),
  marginBottom(Spacing.textSmall),
])
