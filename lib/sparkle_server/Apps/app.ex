defmodule SparkleServer.Apps.App do
  use Ecto.Schema
  import Ecto.Changeset

  schema "apps" do
    field(:description, :string)
    field(:name, :string)
    field(:slug, :string)
    field(:details, :string)
    field(:image_icon, :binary)
    field(:image_one, :binary)
    field(:image_two, :binary)
    field(:image_three, :binary)

    timestamps()

    has_many(:versions, SparkleServer.Apps.Version)
    has_many(:system_profiles, SparkleServer.Profiles.SystemProfile)
    has_many(:system_profile_summaries, SparkleServer.Profiles.SystemProfileSummary)
  end

  @doc false
  def changeset(app, attrs) do
    app
    |> cast(attrs, [
      :name,
      :slug,
      :description,
      :details,
      :image_icon,
      :image_one,
      :image_two,
      :image_three
    ])
    |> validate_required([:name, :slug, :description])
    |> unique_constraint(:slug, name: "apps_slug_uindex")
  end
end
