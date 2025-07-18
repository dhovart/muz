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
  shadows: [
    "none",
    "0px 2px 4px rgba(0, 0, 0, 0.1)",
    "0px 4px 8px rgba(0, 0, 0, 0.1)",
    "0px 6px 12px rgba(0, 0, 0, 0.10)",
    "0px 8px 16px rgba(0, 0, 0, 0.10)",
    "0px 10px 20px rgba(0, 0, 0, 0.10)",
    "0px 12px 24px rgba(0, 0, 0, 0.10)",
    "0px 14px 28px rgba(0, 0, 0, 0.10)",
    "0px 16px 32px rgba(0, 0, 0, 0.10)",
    "0px 18px 36px rgba(0, 0, 0, 0.10)",
    "0px 20px 40px rgba(0, 0, 0, 0.10)",
    "0px 22px 44px rgba(0, 0, 0, 0.10)",
    "0px 24px 48px rgba(0, 0, 0, 0.10)",
    "0px 26px 52px rgba(0, 0, 0, 0.10)",
    "0px 28px 56px rgba(0, 0, 0, 0.10)",
    "0px 30px 60px rgba(0, 0, 0, 0.10)",
    "0px 32px 64px rgba(0, 0, 0, 0.10)",
  ],
  typography: {
    fontFamily: "Inter, Helvetica, Arial, sans-serif",
  },
}

let theme = Theme.create(themeOptions)
