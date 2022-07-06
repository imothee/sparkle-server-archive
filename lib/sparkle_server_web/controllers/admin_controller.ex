defmodule SparkleServerWeb.AdminController do
  use SparkleServerWeb, :controller
  require Logger

  alias SparkleServer.Apps
  alias SparkleServer.Apps.{App, Version}
  alias SparkleServer.Cms
  alias SparkleServer.Cms.Settings
  alias SparkleServer.Profiles

  def index(conn, _params) do
    with apps <- Apps.list_apps() do
      render(conn, "index.html",
        page_title: conn.assigns.settings.site_name,
        apps: apps
      )
    end
  end

  def settings_edit(conn, _params) do
    render(conn, "settings_edit.html", changeset: Settings.changeset(conn.assigns.settings, %{}))
  end

  def settings_update(conn, %{"settings" => params}) do
    case Cms.update_settings(conn.assigns.settings, params) do
      {:ok, _} ->
        conn
        |> put_flash(:info, "Updated site settings.")
        |> redirect(to: "/admin")

      {:error, changeset} ->
        render(conn, "settings_edit.html", changeset: changeset)
    end
  end

  def apps_new(conn, _params) do
    render(conn, "apps_new.html", changeset: App.changeset(%App{}, %{}))
  end

  def apps_create(conn, %{"app" => params}) do
    case Apps.create_app(params) do
      {:ok, _} ->
        conn
        |> put_flash(:info, "Created app.")
        |> redirect(to: "/admin")

      {:error, changeset} ->
        render(conn, "apps_new.html", changeset: changeset)
    end
  end

  def apps_edit(conn, %{"id" => id}) do
    app = Apps.get_app!(id)
    render(conn, "apps_edit.html", app: app, changeset: App.changeset(app, %{}))
  end

  def apps_update(conn, %{"id" => id, "app" => params}) do
    app = Apps.get_app!(id)

    case Apps.update_app(app, params) do
      {:ok, _} ->
        conn
        |> put_flash(:info, "Updated app.")
        |> redirect(to: "/admin")

      {:error, changeset} ->
        render(conn, "apps_edit.html", app: app, changeset: changeset)
    end
  end

  def apps_versions_index(conn, %{"id" => id}) do
    app = Apps.get_app!(id)
    render(conn, "apps_versions_index.html", app: app)
  end

  def apps_versions_new(conn, %{"id" => id}) do
    app = Apps.get_app!(id)

    render(conn, "apps_versions_new.html",
      app: app,
      changeset: Version.changeset(%Version{}, %{app: app})
    )
  end

  def apps_versions_create(conn, %{"id" => id, "version" => params}) do
    app = Apps.get_app!(id)

    case Apps.create_version(app, params) do
      {:ok, _} ->
        conn
        |> put_flash(:info, "Created version.")
        |> redirect(
          to: SparkleServerWeb.Router.Helpers.admin_path(conn, :apps_versions_index, app.id)
        )

      {:error, changeset} ->
        render(conn, "apps_versions_new.html", changeset: changeset)
    end
  end

  def apps_versions_edit(conn, %{"id" => id, "version_id" => version_id}) do
    app = Apps.get_app!(id)
    version = Apps.get_version!(version_id)

    render(conn, "apps_versions_edit.html",
      app: app,
      version: version,
      changeset: Version.changeset(version, %{})
    )
  end

  def apps_versions_update(conn, %{"id" => id, "version_id" => version_id, "version" => params}) do
    app = Apps.get_app!(id)
    version = Apps.get_version!(version_id)

    case Apps.update_version(version, params) do
      {:ok, _} ->
        conn
        |> put_flash(:info, "Updated version.")
        |> redirect(
          to: SparkleServerWeb.Router.Helpers.admin_path(conn, :apps_versions_index, app.id)
        )

      {:error, changeset} ->
        render(conn, "apps_versions_edit.html", app: app, version: version, changeset: changeset)
    end
  end

  def apps_metrics_index(conn, %{"id" => id}) do
    app = Apps.get_app!(id)

    charts = %{
      users: "Users",
      cpu64bit: "CPU 64 Bit",
      ncpu: "Number CPU",
      app_version: "App Version",
      cpu_freq_mhz: "CPU Freq (Mhz)",
      cputype: "CPU Type",
      cpusubtype: "CPU Subtype",
      model: "Model",
      ram_mb: "RAM (mb)",
      os_version: "OS Version",
      lang: "Lang"
    }

    render(conn, "metrics.html", app: app, charts: charts)
  end

  def apps_metrics_summaries_index(conn, %{"id" => id}) do
    app = Apps.get_app!(id)
    summaries = Profiles.get_summaries_by_app(app)

    render(conn, "apps_metrics_summaries_index.json", summaries: summaries)
  end
end
