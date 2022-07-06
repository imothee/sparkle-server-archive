defmodule SparkleServer.Profiles.SystemProfileSummary do
  use Ecto.Schema
  import Ecto.Changeset

  schema "system_profile_summaries" do
    field(:counts, :map)
    field(:date_end, :utc_datetime)
    field(:date_start, :utc_datetime)
    field(:period, Ecto.Enum, values: [:weekly])

    timestamps()

    belongs_to(:app, SparkleServer.Apps.App)
  end

  @doc false
  def changeset(system_profile_summary, attrs) do
    system_profile_summary
    |> cast(attrs, [:app_id, :period, :date_start, :date_end, :counts])
    |> validate_required([:app_id, :period, :date_start, :date_end, :counts])
  end
end
