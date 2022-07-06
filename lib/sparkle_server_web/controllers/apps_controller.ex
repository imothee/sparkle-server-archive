defmodule SparkleServerWeb.AppsController do
  use SparkleServerWeb, :controller

  alias SparkleServer.Apps

  def show(conn, %{"slug" => slug}) do
    app = Apps.get_by_slug!(slug)

    render(conn, "show.html", app: app)
  end

  def images_show(conn, %{"id" => id, "image" => image}) do
    app = Apps.get_app!(id)

    case Apps.get_image(app, image) do
      nil ->
        render(conn, "404.html")

      file ->
        conn
        |> put_resp_content_type("text/jpeg")
        |> put_resp_header("content-disposition", "attachment; filename=#{image}.jpg")
        |> put_root_layout(false)
        |> put_layout(false)
        |> Plug.Conn.send_resp(:ok, file)
    end
  end
end
