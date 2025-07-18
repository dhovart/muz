open CssJs

// Base colors from theme
let primary = hex("c9516b")
let secondary = hex("4b4d7f")
let text = hex("151715")
let backgroundColor = hex("f0ebe6")

// Extended palette derived from theme colors
let primaryLight = hex("d4708a")
let primaryDark = hex("b0354f")
let secondaryLight = hex("5c5d94")
let secondaryDark = hex("3a3c66")

// Neutral colors that complement the beige background
let surface = hex("ffffff")
let surfaceVariant = hex("f5f2ed")
let outline = hex("c7c4bf")
let outlineVariant = hex("ddd9d4")

// Semantic colors
let error = hex("d32f2f")
let warning = hex("ed6c02")
let info = secondary
let success = hex("2e7d32")

// Text variations
let textSecondary = hex("6c6c6c")
let textDisabled = hex("9e9e9e")
let textOnPrimary = hex("ffffff")
let textOnSecondary = hex("ffffff")

// Hover and interaction states
let hoverColor = hex("e8e5e0")
let pressed = hex("ddd9d4")
let selected = primaryLight
let selectedHover = hex("d97ea0")

// Queue-specific colors
let queueCurrent = primary
let queueCurrentHover = primaryDark
let queueItem = surfaceVariant
let queueItemHover = hoverColor

let unpackHex = (color: CssJs.Types.Color.t) => {
  switch color {
  | #hex(value) => value
  | _ => "000000" // fallback
  }
}

let hexString = (color: CssJs.Types.Color.t) => {
  "#" ++ unpackHex(color)
}
