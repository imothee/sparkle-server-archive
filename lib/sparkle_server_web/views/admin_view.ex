defmodule SparkleServerWeb.AdminView do
  use SparkleServerWeb, :view

  def render("apps_metrics_summaries_index.json", %{summaries: summaries}) do
    %{
      summaries:
        Enum.map(summaries, fn summary ->
          %{
            period: summary.period,
            date_start: summary.date_start,
            date_end: summary.date_end,
            counts: summary.counts
          }
        end)
    }
  end
end
