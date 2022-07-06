defmodule SparkleServer.Repo.Migrations.CreateVersions do
  use Ecto.Migration

  def change do
    create table(:versions) do
      add :version, :string
      add :build, :string
      add :min_system_version, :string
      add :description, :string
      add :url, :string
      add :dsa_signature, :string
      add :ed_signature, :string
      add :length, :string
      add :app_id, references(:apps, on_delete: :nothing)

      timestamps()
    end

    create index(:versions, [:app_id])
  end
end
