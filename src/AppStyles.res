open CssJs

let appContainer = style([height(vh(100.)), display(flexBox), flexDirection(column)])

let contentArea = style([flexGrow(1.0), overflowY(#auto), overflowX(hidden), paddingTop(rem(4.))]) // Space for the fixed menu
