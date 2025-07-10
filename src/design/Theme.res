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
      main: "#c9516b",
    },
    secondary: {
      main: "#4b4d7f",
    },
    text: {
      primary: "#151715",
    },
    background: {
      default: "#f0ebe6",
    },
  },
  shadows: ["none"],
  typography: {
    fontFamily: "Inter, Helvetica, Arial, sans-serif",
  },
}

let theme = Theme.create(themeOptions)
