module Command = {
  type t = Play | Pause | Next | Previous | SetVolume(float) | Seek(float)

  let toJsonPayload = (command: t) => {
    let commandName = switch command {
    | Play => Js.Json.string("Play")
    | Pause => Js.Json.string("Pause")
    | Next => Js.Json.string("Next")
    | Previous => Js.Json.string("Previous")
    | SetVolume(_) => Js.Json.string("SetVolume")
    | Seek(_) => Js.Json.string("Seek")
    }

    let contents = switch command {
    | SetVolume(vol) =>
      Js.Json.object_(
        Js.Dict.fromArray([("command", commandName), ("volume", Js.Json.number(vol))]),
      )
    | Seek(position) =>
      Js.Json.object_(
        Js.Dict.fromArray([("command", commandName), ("position", Js.Json.number(position))]),
      )
    | _ => Js.Json.object_(Js.Dict.fromArray([("command", commandName)]))
    }

    Js.Json.object_(Js.Dict.fromArray([("payload", contents)]))
  }
}