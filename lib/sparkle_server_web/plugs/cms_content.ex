defmodule SparkleServerWeb.Plugs.CmsContent do
  import Plug.Conn

  alias SparkleServer.Cms
  alias SparkleServer.Cms.Settings

  def init(_opts) do
  end

  def call(conn, _opts) do
    with %Settings{} = settings <- Cms.get_settings() do
      assign(conn, :settings, settings)
    else
      _ ->
        assign(conn, :settings, %Settings{public_site: false})
    end
  end
end
