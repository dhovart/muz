open PlaybackService

let useSpectrumData = () => {
  let (spectrumData, setSpectrumData) = React.useState(() => [])

  print_endline("Initializing spectrum data hook")
  Js.Console.log(spectrumData)

  React.useEffect(() => {
    let subscription = async () => {
      try {
        let _ = await PlaybackService.subscribeToSpectrum(message => {
          print_endline("Received spectrum data update")
          setSpectrumData(_ => message.spectrumData)
        })
      } catch {
      | Exn.Error(error) => Js.Console.error2("Error subscribing to spectrum data", error)
      }
    }

    subscription()->ignore

    Some(
      () => {
        PlaybackService.unsubscribeFromSpectrum()->ignore
      },
    )
  }, [])

  spectrumData
}
