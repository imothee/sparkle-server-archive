defmodule SparkleServer.Apps do
  @moduledoc """
  The boundary for the Apps system.
  """

  use Ecto.Schema

  alias SparkleServer.Repo
  alias SparkleServer.Apps.{App, Version}

  def list_apps() do
    Repo.all(App)
  end

  def get_by_slug!(slug) do
    Repo.get_by!(App, slug: slug)
    |> Repo.preload([:versions])
  end

  def get_app!(id) do
    Repo.get!(App, id)
    |> Repo.preload([:versions])
  end

  def create_app(attrs) do
    app_params = handle_images(attrs)

    App.changeset(%App{}, app_params)
    |> Repo.insert()
  end

  def update_app(%App{} = app, attrs) do
    app
    |> App.changeset(handle_images(attrs))
    |> Repo.update()
  end

  def get_version!(id) do
    Repo.get!(Version, id)
  end

  def create_version(%App{} = app, attrs) do
    app
    |> Ecto.build_assoc(:versions)
    |> Version.changeset(attrs)
    |> Repo.insert()
  end

  def update_version(%Version{} = version, attrs) do
    version
    |> Version.changeset(attrs)
    |> Repo.update()
  end

  def get_image(app, image) do
    case image do
      "icon" ->
        app.image_icon

      "one" ->
        app.image_one

      "two" ->
        app.image_two

      "three" ->
        app.image_three

      _ ->
        nil
    end
  end

  # Mark: Private

  defp handle_images(attrs) do
    Enum.reduce(
      attrs,
      %{},
      fn {k, v}, map ->
        case k do
          n when n in ["image_icon", "image_one", "image_two", "image_three"] ->
            with {:ok, image} <- handle_image(v) do
              Map.merge(map, %{k => image})
            else
              _ ->
                map
            end

          _ ->
            Map.merge(map, %{k => v})
        end
      end
    )
  end

  defp handle_image(image) do
    case image do
      %{:content_type => "image/jpeg", :path => path} ->
        with {:ok, contents} <- File.read(path) do
          {:ok, contents}
        else
          _ ->
            nil
        end

      _ ->
        nil
    end
  end
end
