open Promise

@react.component
let make = () => {
  let (apps, setApps) = React.useState(_ => [])

  let getApps = () => {
    Fetch.fetch("/api/apps")
    ->then(Fetch.Response.json)
    ->then(data => {
      let response = data->Models.Decode.appsResponse
      setApps(_prev => response.apps)
      resolve()
    })
    ->ignore
  }

  React.useEffect0(() => {
    getApps()
    None
  })

  let appRows = Belt.Array.map(apps, app => {
    <tr key={Belt.Int.toString(app.id)} className="hover:bg-gray-100">
      <td className="p-4 whitespace-nowrap text-base text-gray-900">{React.string(app.name)}</td>
      <td className="p-4 whitespace-nowrap text-base text-gray-900">{React.string(app.slug)}</td>
      <td className="p-4 whitespace-nowrap text-base text-gray-900">{React.string(app.description)}</td>
      <td className="p-4 whitespace-nowrap text-base text-gray-900">
        <a className="text-brand" href={`/admin/apps/${Belt.Int.toString(app.id)}/metrics`}>{React.string("View Metrics")}</a>
      </td>
      <td className="p-4 whitespace-nowrap space-x-2">
        <button className="text-white bg-cyan-600 hover:bg-cyan-700 focus:ring-4 focus:ring-cyan-200 font-medium rounded-lg text-sm inline-flex items-center px-3 py-2 text-center">
          <svg className="mr-2 h-5 w-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
            <path d="M17.414 2.586a2 2 0 00-2.828 0L7 10.172V13h2.828l7.586-7.586a2 2 0 000-2.828z"></path>
            <path fillRule="evenodd" d="M2 6a2 2 0 012-2h4a1 1 0 010 2H4v10h10v-4a1 1 0 112 0v4a2 2 0 01-2 2H4a2 2 0 01-2-2V6z" clipRule="evenodd"></path>
          </svg>
          {React.string("Edit App")}
        </button>
      </td>
    </tr>
  })

  <div className="max-w-[75rem] mx-auto px-4 sm:px-6 lg:px-8 py-20">
    <div className="p-4 bg-white block sm:flex items-center justify-between border-b border-gray-200 lg:mt-1.5">
      <div className="mb-1 w-full">
        <h1 className="text-xl sm:text-2xl font-semibold text-gray-900">{React.string("Apps")}</h1>
      </div>
    </div>
    <div className="flex flex-col">
      <table className="table-fixed min-w-full divide-y divide-gray-200">
        <thead className="bg-light-accent">
          <tr>
            <th scope="col" className="p-4 text-left text-xs font-medium text-dark-shade">{React.string("Name")}</th>
            <th scope="col" className="p-4 text-left text-xs font-medium text-dark-shade">{React.string("Slug")}</th>
            <th scope="col" className="p-4 text-left text-xs font-medium text-dark-shade">{React.string("Description")}</th>
            <th scope="col" className="p-4"></th>
            <th scope="col" className="p-4"></th>
          </tr>
        </thead>
        <tbody className="bg-white divide-y divide-gray-200">
          {React.array(appRows)}
        </tbody>
      </table>
    </div>
  </div>
}