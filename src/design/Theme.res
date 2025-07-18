open Mui

let themeOptions: ThemeOptions.t = {
  breakpoints: {
    keys: ["xs", "sm", "md", "lg", "xl"],
    values: {lg: 1200., md: 960., sm: 600., xl: 1920., xs: 0.},
    unit: "px",
  },
  direction: "ltr",
  components: {
    muiButtonBase: {
      defaultProps: {
        disableRipple: true,
      },
    },
    muiButton: {
      defaultProps: {
        variant: Outlined,
      },
    },
  },
  palette: {
    mode: "light",
    primary: {
      main: Color.hexString(Color.primary),
    },
    secondary: {
      main: Color.hexString(Color.secondary),
    },
    text: {
      primary: Color.hexString(Color.text),
    },
    background: {
      default: Color.hexString(Color.backgroundColor),
    },
  },
  shadows: ["none"],
  typography: {
    fontFamily: "Inter, Helvetica, Arial, sans-serif",
  },
}

let theme = Theme.create(themeOptions)
