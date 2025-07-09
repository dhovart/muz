module Styles = {
  open CssJs

  let button =
    style(. [
      background(white),
      color(black),
      borderRadius(px(1)),
      hover([
        backgroundColor(black),
        color(white),
      ]),
      // selector("&[aria-label]", [
      // ])
    ])
}

let make = props =>
  <button
    {...props}
    className={Styles.button}
  />
