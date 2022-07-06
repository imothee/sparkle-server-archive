defmodule SparkleServer.Repo.Migrations.AddIndexes do
  use Ecto.Migration

  def change do
    create(unique_index(:apps, [:slug]))
    create(index(:system_profile_summaries, [:app_id, :period, :date_start, :date_end]))
    create(unique_index(:versions, [:version, :build]))
  end
end
