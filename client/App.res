module Metrics = {
  @react.component @module("./page/Metrics.jsx")
  external make: (~appId: string) => React.element = "default"
}

@react.component
let make = () => {
  let url = RescriptReactRouter.useUrl()
  {switch url.path {
    | list{"admin", "apps", appId, "metrics"} => <Metrics appId/>
    | list{"admin"} => <Home/>
    | _ => <Home/>
  }}
}