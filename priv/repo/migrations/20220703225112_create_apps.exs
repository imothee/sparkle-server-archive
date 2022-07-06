defmodule SparkleServer.Repo.Migrations.CreateApps do
  use Ecto.Migration

  def change do
    create table(:apps) do
      add :name, :string
      add :slug, :string
      add :description, :string
      add :icon, :string

      timestamps()
    end
  end
end
