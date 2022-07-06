defmodule SparkleServer.Apps.Version do
  use Ecto.Schema
  import Ecto.Changeset

  schema "versions" do
    field(:build, :string)
    field(:description, :string)
    field(:dsa_signature, :string)
    field(:ed_signature, :string)
    field(:length, :string)
    field(:min_system_version, :string)
    field(:url, :string)
    field(:version, :string)

    timestamps()

    belongs_to(:app, SparkleServer.Apps.App)
  end

  @doc false
  def changeset(version, attrs) do
    version
    |> cast(attrs, [
      :version,
      :build,
      :min_system_version,
      :description,
      :url,
      :dsa_signature,
      :ed_signature,
      :length,
      :app_id
    ])
    |> validate_required([
      :version,
      :build,
      :min_system_version,
      :description,
      :url,
      :ed_signature,
      :length,
      :app_id
    ])
    |> assoc_constraint(:app)
  end
end
