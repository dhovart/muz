type t = MusicPlayer | Settings

let toString = route =>
  switch route {
  | MusicPlayer => "player"
  | Settings => "settings"
  }

let fromString = str =>
  switch str {
  | "settings" => Settings
  | _ => MusicPlayer
  }