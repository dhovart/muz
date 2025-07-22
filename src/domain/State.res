type t = Playing | Paused | Stopped

let fromString = (state: string): t => {
  switch state {
  | "Playing" => Playing
  | "Paused" => Paused
  | _ => Stopped
  }
}
