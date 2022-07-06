defmodule SparkleServer.Repo.Migrations.CreateSystemProfiles do
  use Ecto.Migration

  def change do
    create table(:system_profiles) do
      add :app_id, :integer
      add :app_version, :string
      add :cpu64bit, :boolean
      add :ncpu, :integer
      add :cpu_freq_mhz, :string
      add :cputype, :string
      add :cpusubtype, :string
      add :model, :string
      add :ram_mb, :string
      add :os_version, :string
      add :lang, :string

      timestamps()
    end
  end
end
