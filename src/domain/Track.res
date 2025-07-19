type trackMetadata = {
  title: option<string>,
  album: option<string>,
  artist: option<string>,
  album_artist: option<string>,
  track_number: option<int>,
  disc_number: option<int>,
  genre: option<string>,
  year: option<string>,
}

type t = {
  id: string,
  path: string,
  totalFrames: int,
  metadata: trackMetadata,
}

let displayTitle = (track: t) => {
  switch track.metadata.title {
  | Some(title) => title
  | None => "Unknown Track"
  }
}

let displayArtist = (track: t) => {
  switch track.metadata.artist {
  | Some(artist) => artist
  | None => "Unknown Artist"
  }
}
