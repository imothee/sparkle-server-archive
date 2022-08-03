defmodule SparkleServer.Profiles do
  @moduledoc """
  The boundary for the Profiles system.
  """

  use Ecto.Schema
  use Timex
  import Ecto.Query
  require Logger

  alias SparkleServer.Repo
  alias SparkleServer.Apps
  alias SparkleServer.Profiles.{SystemProfile, SystemProfileSummary}

  # Mark: SystemProfile

  def create_system_profile(attrs \\ %{}) do
    %SystemProfile{}
    |> SystemProfile.changeset(attrs)
    |> Repo.insert()
  end

  def list_system_profiles(app) do
    SystemProfile
    |> where([s], s.app_id == ^app.id)
    |> Repo.all()
  end

  def list_system_profiles_between(app, date_start, date_end) do
    SystemProfile
    |> where(
      [s],
      s.app_id == ^app.id and s.inserted_at >= ^date_start and s.inserted_at < ^date_end
    )
    |> order_by([s], s.inserted_at)
    |> Repo.all()
  end

  def delete_system_profiles(system_profiles) do
    SystemProfile
    |> where([s], s.id in ^Enum.map(system_profiles, fn sp -> sp.id end))
    |> Repo.delete_all()
  end

  # Mark: SystemProfileSummary

  def get_summaries_by_app(app) do
    SystemProfileSummary
    |> where([s], s.app_id == ^app.id)
    |> Repo.all()
  end

  def create_system_profile_summary(attrs \\ %{}) do
    %SystemProfileSummary{}
    |> SystemProfileSummary.changeset(attrs)
    |> Repo.insert()
  end

  def update_system_profile_summary(%SystemProfileSummary{} = summary, attrs) do
    summary
    |> SystemProfileSummary.changeset(attrs)
    |> Repo.update()
  end

  def get_summary_by_app_and_date_start(app, date_start) do
    SystemProfileSummary
    |> where([s], s.app_id == ^app.id and s.date_start == ^date_start)
    |> Repo.one()
  end

  # Mark: Task Items

  def summarize_system_profiles() do
    now = Timex.now()
    date_start = Timex.beginning_of_week(now)

    summarize_system_profiles_from_date(date_start)
  end

  def summarize_system_profiles_weeks_ago(weeks) do
    now = Timex.now()
    before = Timex.shift(now, weeks: -weeks)
    date_start = Timex.beginning_of_week(before)

    summarize_system_profiles_from_date(date_start)
  end

  def summarize_system_profiles_from_date(date_start) do
    date_end = Timex.end_of_week(date_start)

    for app <- Apps.list_apps() do
      with {:ok, %SystemProfileSummary{} = summary} <- get_summary(app, date_start, date_end) do
        # Grab all the system_profiles for the date_start
        system_profiles = list_system_profiles_between(app, date_start, date_end)

        counts =
          Map.new(summary.counts, fn {k, v} ->
            if(is_atom(k), do: {k, v}, else: {String.to_atom(k), v})
          end)

        summary_count = get_system_profile_summary_counts(system_profiles, counts)
        Logger.debug(summary_count)

        with {:ok, %SystemProfileSummary{} = _} <-
               update_system_profile_summary(summary, %{counts: summary_count}) do
          delete_system_profiles(system_profiles)
        end
      else
        err -> Logger.warn(err)
      end
    end
  end

  defp get_summary(app, date_start, date_end) do
    case get_summary_by_app_and_date_start(app, date_start) do
      nil ->
        create_system_profile_summary(%{
          app_id: app.id,
          date_start: date_start,
          date_end: date_end,
          period: :weekly,
          counts: %{
            users: 0,
            cpu64bit: %{},
            ncpu: %{},
            app_version: %{},
            cpu_freq_mhz: %{},
            cputype: %{},
            cpusubtype: %{},
            model: %{},
            ram_mb: %{},
            os_version: %{},
            lang: %{}
          }
        })

      summary ->
        {:ok, summary}
    end
  end

  defp get_system_profile_summary_counts(system_profiles, counts) do
    Logger.debug(counts)

    Enum.reduce(
      system_profiles,
      counts,
      fn profile, map ->
        %{
          users: map[:users] + 1,
          cpu64bit:
            if(profile.cpu64bit != nil,
              do:
                count_profile(
                  map[:cpu64bit],
                  if(profile.cpu64bit, do: "true", else: "false")
                ),
              else: map[:cpu64bit]
            ),
          ncpu: count_profile(map[:ncpu], profile.ncpu),
          app_version: count_profile(map[:app_version], profile.app_version),
          cpu_freq_mhz: count_profile(map[:cpu_freq_mhz], profile.cpu_freq_mhz),
          cputype: count_profile(map[:cputype], profile.cputype),
          cpusubtype: count_profile(map[:cpusubtype], profile.cpusubtype),
          model: count_profile(map[:model], profile.model),
          ram_mb: count_profile(map[:ram_mb], profile.ram_mb),
          os_version: count_profile(map[:os_version], profile.os_version),
          lang: count_profile(map[:lang], profile.lang)
        }
      end
    )
  end

  defp count_profile(map, key) do
    case key do
      nil ->
        map

      key ->
        Map.update(map, key, 1, fn count -> count + 1 end)
    end
  end
end
