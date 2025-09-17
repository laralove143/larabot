defmodule Larabot.Interactions do
  alias Larabot.Error
  alias Nostrum.Api.ApplicationCommand

  @callback commands() :: any()

  defmacro __using__(_) do
    quote do
      @app Mix.Project.config()[:app]

      @behaviour Larabot.Interactions

      def command_ids do
        Application.get_env(@app, :command_ids, %{})
      end

      def register do
        guild_id = Application.get_env(@app, :guild_id)
        commands = Enum.map(commands(), fn cmd -> cmd.command() end)

        if guild_id do
          guild_id
          |> ApplicationCommand.bulk_overwrite_guild_commands(commands)
          |> Error.handle()
        end

        {:ok, commands_response} =
          commands
          |> ApplicationCommand.bulk_overwrite_global_commands()
          |> Error.handle()

        command_ids = Map.new(commands_response, &{&1.name, &1.id})

        Application.put_env(@app, :command_ids, command_ids)
      end

      def handle(interaction) do
        command =
          Enum.find(Timezoner.Interactions.commands(), fn cmd ->
            cmd.name() == interaction.data.name
          end)

        command.handle(interaction)
      end
    end
  end
end
