type t = MusicPlayer | Library | Settings

let toString = route =>
  switch route {
  | MusicPlayer => "player"
  | Library => "library"
  | Settings => "settings"
  }

let fromString = str =>
  switch str {
  | "library" => Library
  | "settings" => Settings
  | _ => MusicPlayer
  }