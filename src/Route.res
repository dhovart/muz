type t = MillerColumns | Settings | Visualizer

let toString = route =>
  switch route {
  | MillerColumns => "miller-columns"
  | Settings => "settings"
  | Visualizer => "visualizer"
  }

let fromString = str =>
  switch str {
  | "miller-columns" => MillerColumns
  | "settings" => Settings
  | "visualizer" => Visualizer
  | _ => MillerColumns
  }
