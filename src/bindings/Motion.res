// Motion (formerly Framer Motion) bindings for ReScript

@module("framer-motion") 
external motion: 'a = "motion"

@get external div: 'a => React.component<'b> = "div"

module MotionDiv = {
  @react.component
  let make = (~layout=?, ~layoutId=?, ~animate=?, ~initial=?, ~exit=?, ~transition=?, ~whileHover=?, ~whileTap=?, ~style=?, ~className=?, ~onClick=?, ~children=?, ()) => {
    let motionComponent = motion->div
    React.createElement(motionComponent, {
      "layout": layout,
      "layoutId": layoutId,
      "animate": animate,
      "initial": initial,
      "exit": exit,
      "transition": transition,
      "whileHover": whileHover,
      "whileTap": whileTap,
      "style": style,
      "className": className,
      "onClick": onClick,
      "children": children,
    })
  }
}

module LayoutGroup = {
  @module("framer-motion") @react.component
  external make: (~children: React.element=?) => React.element = "LayoutGroup"
}