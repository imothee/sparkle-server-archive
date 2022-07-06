defmodule SparkleServer.Repo.Migrations.CreateSettings do
  use Ecto.Migration

  def change do
    create table(:settings) do
      add(:public_site, :boolean, default: false, null: false)
      add(:site_name, :string)
      add(:site_description, :string)
      add(:site_keywords, :string)
      add(:author, :string)
      add(:show_apps, :boolean, default: false, null: false)

      timestamps()
    end
  end
end
