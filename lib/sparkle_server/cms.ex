defmodule SparkleServer.Cms do
  @moduledoc """
  The boundary for the Cms system.
  """

  use Ecto.Schema
  require Logger

  alias SparkleServer.Repo
  alias SparkleServer.Cms.Settings

  # Mark: Settings
  def get_settings() do
    Settings
    |> Repo.one()
  end

  def update_settings(%Settings{} = settings, attrs) do
    if Ecto.get_meta(settings, :state) == :built do
      Settings.changeset(settings, attrs)
      |> Repo.insert()
    else
      settings
      |> Settings.changeset(attrs)
      |> Repo.update()
    end
  end
end
