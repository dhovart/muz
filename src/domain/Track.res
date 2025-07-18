type t = {
  id: string,
  path: string,
  title: option<string>,
  totalFrames: int,
}

let displayTitle = (track: t) => {
  switch track.title {
  | Some(title) => title
  | None => "Unknown Track"
  }
}