open PlaybackService

let useSpectrumData = () => {
  let (spectrumData, setSpectrumData) = React.useState(() => [])


  React.useEffect(() => {
    let subscription = async () => {
      try {
        let foo = await PlaybackService.subscribeToSpectrum(message => {
          setSpectrumData(_ => message.spectrumData)
        })
        Js.Console.log2("Subscribed to spectrum data updates", foo)
      } catch {
      | Exn.Error(error) => Js.Console.error2("Error subscribing to spectrum data", error)
      }
    }

    subscription()->ignore

    None
  }, [])

  spectrumData
}
