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
let track = style([
  width(pct(100.)),
  marginBottom(Spacing.sectionMargin),
  maxWidth(rem(25.)),
  padding2(~v=Spacing.md, ~h=Spacing.lg),
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

let controlsContainer = style([
  display(#flex),
  alignItems(#center),
  justifyContent(#center),
  marginBottom(Spacing.sectionMargin),
  padding(Spacing.cardPadding),
  backgroundColor(Color.surface),
  borderRadius(rem(1.5)),
  border(Spacing.borderThin, #solid, Color.outline),
])

let trackInfo = style([
  textAlign(#center),
  marginBottom(Spacing.sectionMargin),
  padding(Spacing.cardPadding),
  backgroundColor(Color.surface),
  borderRadius(rem(1.)),
  border(Spacing.borderThin, #solid, Color.outline),
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
