import Config

# Only in tests, remove the complexity from the password hashing algorithm
config :bcrypt_elixir, :log_rounds, 1

# Configure your database
#
# The MIX_TEST_PARTITION environment variable can be used
# to provide built-in test partitioning in CI environment.
# Run `mix help test` for more information.
config :sparkle_server, SparkleServer.Repo,
  username: "postgres",
  password: "postgres",
  hostname: "localhost",
  database: "sparkle_server_test#{System.get_env("MIX_TEST_PARTITION")}",
  pool: Ecto.Adapters.SQL.Sandbox,
  pool_size: 10

# We don't run a server during test. If one is required,
# you can enable the server option below.
config :sparkle_server, SparkleServerWeb.Endpoint,
  http: [ip: {127, 0, 0, 1}, port: 4002],
  secret_key_base: "o3DphC9jjuoBz8F4VFj4uxta57hoByGs31I5dmIpkLR2abgv0rSMk4Qp/j36uAC5",
  server: false

# In test we don't send emails.
config :sparkle_server, SparkleServer.Mailer, adapter: Swoosh.Adapters.Test

# Print only warnings and errors during test
config :logger, level: :warn

# Initialize plugs at runtime for faster test compilation
config :phoenix, :plug_init_mode, :runtime
