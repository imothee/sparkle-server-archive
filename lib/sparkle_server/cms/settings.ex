defmodule SparkleServer.Cms.Settings do
  use Ecto.Schema
  import Ecto.Changeset

  schema "settings" do
    field(:public_site, :boolean, default: false)
    field(:show_apps, :boolean, default: false)
    field(:site_description, :string)
    field(:site_keywords, :string)
    field(:site_name, :string)
    field(:author, :string)

    timestamps()
  end

  @doc false
  def changeset(settings, attrs) do
    settings
    |> cast(attrs, [
      :author,
      :public_site,
      :site_name,
      :site_description,
      :site_keywords,
      :show_apps
    ])
    |> validate_required([
      :author,
      :public_site,
      :site_name,
      :show_apps
    ])
  end
end
