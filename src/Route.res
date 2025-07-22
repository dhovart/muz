type t = Library | MillerColumns | Settings | Visualizer

let toString = route =>
  switch route {
  | Library => "library"
  | MillerColumns => "miller-columns"
  | Settings => "settings"
  | Visualizer => "visualizer"
  }

let fromString = str =>
  switch str {
  | "library" => Library
  | "miller-columns" => MillerColumns
  | "settings" => Settings
  | "visualizer" => Visualizer
  | _ => Library
  }
