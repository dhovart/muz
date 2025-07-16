module Command = {
  type t = Play | Pause | Next | Previous | SetVolume(float)

  let toJsonPayload = (command: t) => {
    let commandName = switch command {
    | Play => Js.Json.string("Play")
    | Pause => Js.Json.string("Pause")
    | Next => Js.Json.string("Next")
    | Previous => Js.Json.string("Previous")
    | SetVolume(_) => Js.Json.string("SetVolume")
    }

    let contents = switch command {
    | SetVolume(vol) =>
      Js.Json.object_(
        Js.Dict.fromArray([("command", commandName), ("volume", Js.Json.number(vol))]),
      )
    | _ => Js.Json.object_(Js.Dict.fromArray([("command", commandName)]))
    }

    Js.Json.object_(Js.Dict.fromArray([("payload", contents)]))
  }
}
