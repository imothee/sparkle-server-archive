defmodule SparkleServerWeb.PageController do
  use SparkleServerWeb, :controller

  alias SparkleServer.Apps

  def index(conn, _params) do
    meta_attrs = [
      %{name: "keywords", content: conn.assigns.settings.site_keywords},
      %{name: "description", content: conn.assigns.settings.site_description}
    ]

    if conn.assigns.settings.public_site == false do
      render(conn, "404.html")
    else
      with apps <- Apps.list_apps() do
        render(conn, "index.html",
          meta_attrs: meta_attrs,
          page_title: conn.assigns.settings.site_name,
          apps: apps
        )
      end
    end
  end
end
