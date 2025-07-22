type selectedPath = {
  artist: option<string>,
  album: option<string>,
}

type state = {
  selectedPath: selectedPath,
  artistsData: option<Js.Dict.t<Js.Dict.t<array<Track.t>>>>,
}

type action =
  | SetArtistsData(Js.Dict.t<Js.Dict.t<array<Track.t>>>)
  | SelectArtist(string)
  | SelectAlbum(string)

let reducer = (state, action) => {
  switch action {
  | SetArtistsData(data) => {...state, artistsData: Some(data)}
  | SelectArtist(artist) => {
      ...state,
      selectedPath: {artist: Some(artist), album: None},
    }
  | SelectAlbum(album) => {
      ...state,
      selectedPath: {...state.selectedPath, album: Some(album)},
    }
  }
}

let initialState = {
  selectedPath: {artist: None, album: None},
  artistsData: None,
}

@react.component
let make = () => {
  let (state, dispatch) = React.useReducer(reducer, initialState)
  let {currentTrack}: PlayerContext.playerState = PlayerContext.usePlayer()

  React.useEffect0(() => {
    switch state.artistsData {
    | None =>
      LibraryService.getAlbumsByArtist()
      ->Promise.then(albumsByArtist => {
        dispatch(SetArtistsData(albumsByArtist))
        Promise.resolve()
      })
      ->Promise.catch(error => {
        Js.Console.error2("Failed to load library data:", error)
        Promise.resolve()
      })
      ->ignore
    | Some(_) => ()
    }
    None
  })

  let getArtists = () => {
    switch state.artistsData {
    | Some(data) => {
        let artists = Js.Dict.keys(data)
        Js.Array2.sortInPlaceWith(artists, compare)->ignore
        artists
      }
    | None => []
    }
  }

  let getAlbumsForArtist = (artist: string) => {
    switch state.artistsData {
    | Some(data) =>
      switch Js.Dict.get(data, artist) {
      | Some(albums) => {
          let albumNames = Js.Dict.keys(albums)
          Js.Array2.sortInPlaceWith(albumNames, compare)->ignore
          albumNames
        }
      | None => []
      }
    | None => []
    }
  }

  let getTracksForAlbum = (artist: string, album: string) => {
    switch state.artistsData {
    | Some(data) =>
      switch Js.Dict.get(data, artist) {
      | Some(albums) =>
        switch Js.Dict.get(albums, album) {
        | Some(tracks) => tracks
        | None => []
        }
      | None => []
      }
    | None => []
    }
  }

  let artists = getArtists()
  let albums = switch state.selectedPath.artist {
  | Some(artist) => getAlbumsForArtist(artist)
  | None => []
  }
  let tracks = switch (state.selectedPath.artist, state.selectedPath.album) {
  | (Some(artist), Some(album)) => getTracksForAlbum(artist, album)
  | _ => []
  }

  <div className={MillerColumnsViewStyles.container}>
    <div className={MillerColumnsViewStyles.columnsContainer}>
      <MillerColumn
        title="Artists"
        items={artists->Belt.Array.map(artist => (artist, artist))}
        selectedItem={state.selectedPath.artist}
        onSelect={artist => dispatch(SelectArtist(artist))}
        currentTrack={currentTrack}
      />
      {switch state.selectedPath.artist {
      | Some(_) =>
        <MillerColumn
          title="Albums"
          items={albums->Belt.Array.map(album => (album, album))}
          selectedItem={state.selectedPath.album}
          onSelect={album => dispatch(SelectAlbum(album))}
          currentTrack={currentTrack}
        />
      | None => React.null
      }}
      {switch (state.selectedPath.artist, state.selectedPath.album) {
      | (Some(artist), Some(album)) =>
        <MillerColumnTracks
          title="Tracks" tracks={tracks} artist={artist} album={album} currentTrack={currentTrack}
        />
      | _ => React.null
      }}
    </div>
  </div>
}
