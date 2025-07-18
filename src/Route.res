type t = MusicPlayer | Library | Settings | Visualizer

let toString = route =>
  switch route {
  | MusicPlayer => "player"
  | Library => "library"
  | Settings => "settings"
  | Visualizer => "visualizer"
  }

let fromString = str =>
  switch str {
  | "library" => Library
  | "settings" => Settings
  | "visualizer" => Visualizer
  | _ => MusicPlayer
  }