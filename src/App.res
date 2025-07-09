@react.component
let make = () => {
  let (count, setCount) = React.useState(() => 0)

  <div>
    <h1> {"Tauri + Rescript + Vite + React"->React.string} </h1>
    <p>
      {React.string("This is a simple template for a Vite project using ReScript.")}
    </p>
    <h2> {React.string("Fast Refresh Test")} </h2>
    <Button onClick={_ => setCount(count => count + 1)}>
      {React.string(`count is ${count->Int.toString}`)}
    </Button>
    <p>
      {React.string("Edit ")}
      <code> {React.string("src/App.res")} </code>
      {React.string(" and save to test Fast Refresh.")}
    </p>
  </div>
}
