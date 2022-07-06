defmodule SparkleServer.Repo.Migrations.CreateSystemProfileSummaries do
  use Ecto.Migration

  def change do
    create table(:system_profile_summaries) do
      add :period, :string
      add :date_start, :utc_datetime
      add :date_end, :utc_datetime
      add :counts, :map
      add :app_id, references(:apps, on_delete: :nothing)

      timestamps()
    end

    create index(:system_profile_summaries, [:app_id])
  end
end
