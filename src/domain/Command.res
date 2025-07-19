module Command = {
  type t = Play | Pause | Next | Previous | SetVolume(float) | Seek(int)

  let toJsonPayload = (command: t) => {
    let (commandName, extraFields) = switch command {
    | Play => ("Play", [])
    | Pause => ("Pause", [])
    | Next => ("Next", [])
    | Previous => ("Previous", [])
    | SetVolume(vol) => ("SetVolume", [("volume", Js.Json.number(vol))])
    | Seek(position) => {
        Js.log2("Creating Seek command with position:", position)
        ("Seek", [("seekPosition", Js.Json.number(Float.fromInt(position)))])
      }
    }

    let baseFields = [("command", Js.Json.string(commandName))]
    let allFields = Array.concat(baseFields, extraFields)
    let payload = Js.Json.object_(Js.Dict.fromArray(allFields))
    let finalPayload = Js.Json.object_(Js.Dict.fromArray([("payload", payload)]))
    
    Js.log2("Final JSON payload:", finalPayload)
    finalPayload
  }
}
