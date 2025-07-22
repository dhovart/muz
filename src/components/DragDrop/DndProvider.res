type trackItem = string

module TrackItem = {
  type t = trackItem
  let eq = (x1, x2) => x1 == x2
  let cmp = compare
}

module QueueContainer = Dnd.MakeSingletonContainer()
module QueueDnd = Dnd.Make(TrackItem, QueueContainer)

@react.component
let make = (~children, ~onReorder) => {
  <QueueDnd.DndManager onReorder={onReorder}> {children} </QueueDnd.DndManager>
}
