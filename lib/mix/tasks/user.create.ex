defmodule Mix.Tasks.User.Create do
  use Mix.Task

  alias SparkleServer.Repo
  alias SparkleServer.Accounts
  alias SparkleServer.Accounts.User

  @requirements ["app.config"]
  @requirements ["app.start"]
  @shortdoc "Creates a new user"

  @impl Mix.Task
  def run(args) do
    password = Base.url_encode64(:crypto.strong_rand_bytes(24), padding: false)
    user_params = %{email: Enum.at(args, 0), password: password}

    case Accounts.register_user(user_params) do
      {:ok, user} ->
        user
        |> User.confirm_changeset()
        |> Repo.update()

        Mix.shell().info("Created user #{user.email} with password #{password}")

      {:error, %Ecto.Changeset{} = changeset} ->
        Mix.shell().info("Failed to create user with error #{changeset.errors}")
    end
  end
end
