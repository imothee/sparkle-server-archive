defmodule SparkleServer.Repo.Migrations.AddContentToApps do
  use Ecto.Migration

  def change do
    alter table(:apps) do
      remove(:icon)
      add(:details, :text)
      add(:image_icon, :binary)
      add(:image_one, :binary)
      add(:image_two, :binary)
      add(:image_three, :binary)
    end
  end
end
