open PlaybackService

let useProgressData = () => {
  let (position, setPosition) = React.useState(() => 0.0)
  let (framesPlayed, setFramesPlayed) = React.useState(() => 0)

  React.useEffect(() => {
    let subscription = async () => {
      try {
        let _ = await PlaybackService.subscribeToProgress(message => {
          setPosition(_ => message.position)
          setFramesPlayed(_ => message.framesPlayed)
        })
      } catch {
      | Exn.Error(error) => Js.Console.error2("Error subscribing to progress data", error)
      }
    }

    subscription()->ignore

    Some(() => {
      PlaybackService.unsubscribeFromProgress()->ignore
    })
  }, [])

  (position, framesPlayed)
}
