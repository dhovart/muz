// Tauri API bindings module

// Core types
type eventPayload<'a> = {payload: 'a}
type channelType<'a> = {mutable onmessage: 'a => unit}
type listenEventArg<'a> = {event: eventPayload<'a>}
type listenEvent<'a> = listenEventArg<'a> => unit
type listenCallback<'listenEvent> = 'listenEvent => unit

// External bindings
@module("@tauri-apps/api/core")
external invoke: (string, 'a) => Promise.t<'b> = "invoke"

@module("@tauri-apps/api/event")
external listen: (string, listenCallback<'a>) => promise<unit => unit> = "listen"

@module("@tauri-apps/api/core") @new
external channel: unit => channelType<'a> = "Channel"

@module("@tauri-apps/plugin-dialog")
external _open: {..} => Promise.t<Nullable.t<string>> = "open"

// Convenience functions
let invokeCommand = (command: string, payload: 'a): Promise.t<'b> => {
  invoke(command, payload)
}

let listenToEvent = (eventName: string, callback: 'a => unit): promise<unit => unit> => {
  listen(eventName, event => {
    callback(event.payload)
  })
}

// Dialog functions
let openFolderDialog = (~defaultPath=?, ()): Promise.t<Nullable.t<string>> => {
  let baseOptions = {
    "directory": true,
    "multiple": false,
  }

  let withDefaultPath = switch defaultPath {
  | Some(p) => Js.Obj.assign(baseOptions, {"defaultPath": p})
  | None => baseOptions
  }

  _open(withDefaultPath)
}
