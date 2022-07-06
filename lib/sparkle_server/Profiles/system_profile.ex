defmodule SparkleServer.Profiles.SystemProfile do
  use Ecto.Schema
  import Ecto.Changeset

  schema "system_profiles" do
    field :app_version, :string
    field :cpu64bit, :boolean, default: false
    field :cpu_freq_mhz, :string
    field :cpusubtype, :string
    field :cputype, :string
    field :lang, :string
    field :model, :string
    field :ncpu, :integer
    field :os_version, :string
    field :ram_mb, :string

    timestamps()

    belongs_to :app, SparkleServer.Apps.App
  end

  @doc false
  def changeset(system_profile, attrs) do
    system_profile
    |> cast(attrs, [
      :app_id,
      :app_version,
      :cpu64bit,
      :ncpu,
      :cpu_freq_mhz,
      :cputype,
      :cpusubtype,
      :model,
      :ram_mb,
      :os_version,
      :lang
    ])
    |> validate_required([
      :app_id
    ])
  end
end
