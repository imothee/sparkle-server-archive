defmodule SparkleServer.Repo do
  use Ecto.Repo,
    otp_app: :sparkle_server,
    adapter: Ecto.Adapters.Postgres
end
