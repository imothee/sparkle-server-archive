defmodule SparkleServerWeb.AppcastController do
  use SparkleServerWeb, :controller
  require Logger

  alias SparkleServer.{Apps, Profiles}

  def show(conn, params) do
    app =
      params["slug"]
      |> String.split(".")
      |> List.first()
      |> Apps.get_by_slug!()

    # Create the system profile async, we don't really care if it fails
    case Profiles.create_system_profile(system_profile_params(params, app)) do
      {:ok, _} ->
        Logger.debug("Created profile")

      {:error, %Ecto.Changeset{} = changeset} ->
        Logger.warn(changeset.errors)
    end

    conn
    |> put_resp_content_type("text/xml")
    |> render("show.xml", app: app)
  end

  defp system_profile_params(params, app) do
    %{
      app_id: app.id,
      cpu64bit: params["cpu64bit"],
      ncpu: params["ncpu"],
      app_version: params["appVersion"],
      cpu_freq_mhz: params["cpuFreqMHz"],
      cputype: params["cputype"],
      cpusubtype: params["cpusubtype"],
      model: params["model"],
      ram_mb: params["ramMB"],
      os_version: params["osVersion"],
      lang: params["lang"]
    }
  end
end
