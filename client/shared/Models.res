type app = {
  id: int,
  name: string,
  slug: string,
  description: string,
  icon: option<string>,
  created_at: string,
  updated_at: string,
}

type appsResponse = {
  apps: array<app>,
}

type metricModel = {
  id: int,
  app_id: int,
  date: string,
  profile_key: string,
  profile_value: string,
  count: int,
  created_at: string,
  updated_at: string,
}

type metricsResponse = {
  metrics: array<metricModel>,
}

module Decode = {
  open Json.Decode
  let app = json => {
    id: json |> field("id", int),
    name: json |> field("name", string),
    slug: json |> field("slug", string),
    description: json |> field("description", string),
    icon: json |> optional(field("icon", string)),
    created_at: json |> field("created_at", string),
    updated_at: json |> field("updated_at", string),
  }

  let appsResponse = json => {
    apps: json |> field("apps", array(app)),
  }

  let metric = json => {
    id: json |> field("id", int),
    app_id: json |> field("app_id", int),
    date: json |> field("date", string),
    profile_key: json |> field("profile_key", string),
    profile_value: json |> field("profile_value", string),
    count: json |> field("count", int),
    created_at: json |> field("created_at", string),
    updated_at: json |> field("updated_at", string),
  }

  let metricsResponse = json => {
    metrics: json |> field("metrics", array(metric)),
  }
}