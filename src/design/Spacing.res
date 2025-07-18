// Unified spacing system for consistent design
// Based on 8px base unit (0.5rem) with golden ratio progression

open CssJs

// Base spacing unit (8px = 0.5rem)
let baseUnit = rem(0.5)

// Micro spacings for fine adjustments
let xs = rem(0.25)    // 4px - for letter spacing, fine borders
let sm = rem(0.5)     // 8px - small gaps, tight padding

// Standard spacings
let md = rem(1.0)     // 16px - default padding, standard margins
let lg = rem(1.5)     // 24px - section padding, comfortable spacing
let xl = rem(2.0)     // 32px - component margins, generous spacing

// Macro spacings for layout
let xxl = rem(3.0)    // 48px - large section separation
let xxxl = rem(4.0)   // 64px - major layout spacing

// Semantic aliases for common use cases
let borderThin = rem(0.0625)   // 1px - thin borders
let borderMedium = rem(0.125)  // 2px - medium borders
let borderThick = rem(0.1875)  // 3px - thick borders, focus indicators

// Component-specific spacing
let containerPadding = lg     // 24px - container padding
let sectionMargin = xl        // 32px - section margins
let elementGap = sm          // 8px - gap between elements
let buttonPadding = md       // 16px - button internal padding
let cardPadding = lg         // 24px - card internal padding

// Typography spacing
let textSmall = rem(0.25)         // 4px - small text margins
let textMedium = rem(0.5)         // 8px - medium text margins
let textLarge = rem(1.0)          // 16px - large text margins

// Layout spacing
let layoutGutter = xl        // 32px - layout gutters
let layoutSection = xxl     // 48px - section separation