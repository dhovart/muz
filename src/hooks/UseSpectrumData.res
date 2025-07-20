open PlaybackService

let useSpectrumData = () => {
  let (spectrumData, setSpectrumData) = React.useState(() => [])

  React.useEffect(() => {
    let subscription = async () => {
      try {
        let _ = await PlaybackService.subscribeToSpectrum(message => {
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
