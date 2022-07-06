defmodule SparkleServerWeb.Router do
  use SparkleServerWeb, :router

  import SparkleServerWeb.UserAuth

  pipeline :browser do
    plug(:accepts, ["html"])
    plug(:fetch_session)
    plug(:fetch_live_flash)
    plug(:put_root_layout, {SparkleServerWeb.LayoutView, :root})
    plug(:protect_from_forgery)
    plug(:put_secure_browser_headers)
    plug(:fetch_current_user)
    plug(SparkleServerWeb.Plugs.CmsContent)
  end

  pipeline :api do
    plug(:accepts, ["json"])
  end

  pipeline :appcast do
    plug(:accepts, ["html", "xml"])
    plug(:put_root_layout, false)
    plug(:put_layout, false)
  end

  # Other scopes may use custom stacks.
  # scope "/api", SparkleServerWeb do
  #   pipe_through :api
  # end

  # Enables LiveDashboard only for development
  #
  # If you want to use the LiveDashboard in production, you should put
  # it behind authentication and allow only admins to access it.
  # If your application does not have an admins-only section yet,
  # you can use Plug.BasicAuth to set up some basic authentication
  # as long as you are also using SSL (which you should anyway).
  if Mix.env() in [:dev, :test] do
    import Phoenix.LiveDashboard.Router

    scope "/" do
      pipe_through(:browser)

      live_dashboard("/dashboard", metrics: SparkleServerWeb.Telemetry)
    end
  end

  # Enables the Swoosh mailbox preview in development.
  #
  # Note that preview only shows emails that were sent by the same
  # node running the Phoenix server.
  if Mix.env() == :dev do
    scope "/dev" do
      pipe_through(:browser)

      forward("/mailbox", Plug.Swoosh.MailboxPreview)
    end
  end

  scope "/", SparkleServerWeb do
    pipe_through(:browser)

    get("/", PageController, :index)
    get("/apps/:slug", AppsController, :show)
    get("/apps/:id/images/:image", AppsController, :images_show)
  end

  scope "/updates", SparkleServerWeb do
    pipe_through(:appcast)

    get("/:slug", AppcastController, :show)
  end

  scope "/", SparkleServerWeb do
    pipe_through([:browser, :require_authenticated_user])

    get("/admin", AdminController, :index)

    get("/admin/settings/edit", AdminController, :settings_edit)
    post("/admin/settings", AdminController, :settings_update)
    put("/admin/settings", AdminController, :settings_update)

    get("/admin/apps/new", AdminController, :apps_new)
    post("/admin/apps", AdminController, :apps_create)
    get("/admin/apps/:id/edit", AdminController, :apps_edit)
    put("/admin/apps/:id", AdminController, :apps_update)

    get("/admin/apps/:id/versions", AdminController, :apps_versions_index)
    get("/admin/apps/:id/versions/new", AdminController, :apps_versions_new)
    post("/admin/apps/:id/versions", AdminController, :apps_versions_create)
    get("/admin/apps/:id/versions/:version_id/edit", AdminController, :apps_versions_edit)
    put("/admin/apps/:id/versions/:version_id", AdminController, :apps_versions_update)

    get("/admin/apps/:id/metrics", AdminController, :apps_metrics_index)
    get("/admin/apps/:id/metrics/summaries", AdminController, :apps_metrics_summaries_index)
  end

  ## Authentication routes

  scope "/", SparkleServerWeb do
    pipe_through([:browser, :redirect_if_user_is_authenticated])

    get("/users/log_in", UserSessionController, :new)
    post("/users/log_in", UserSessionController, :create)
  end

  scope "/", SparkleServerWeb do
    pipe_through([:browser])

    delete("/users/log_out", UserSessionController, :delete)
  end
end
