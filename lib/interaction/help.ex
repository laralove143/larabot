defmodule Larabot.Interaction.Help do
  @callback title() :: any()
  @callback description() :: any()
  @callback components() :: any()
  @callback homepage() :: any()

  defmacro __using__(_) do
    quote do
      @behaviour Larabot.Interaction

      alias Larabot.Component
      alias Larabot.Error
      alias Larabot.InteractionResponse
      alias Nostrum.Api.Interaction
      alias Nostrum.Constants.ApplicationCommandType
      alias Nostrum.Struct.Component.ActionRow
      alias Nostrum.Struct.Component.Button

      @impl Larabot.Interaction
      def name, do: "help"

      @impl Larabot.Interaction
      def command do
        %{
          name: name(),
          description: "Get information about the bot",
          type: ApplicationCommandType.chat_input()
        }
      end

      @impl Larabot.Interaction
      def handle(interaction) do
        response =
          InteractionResponse.channel_message_with_source(
            [title_section(title(), description())] ++
              components() ++ [footer_section(), action_row(homepage())]
          )

        interaction
        |> Interaction.create_response(response)
        |> Error.handle()
      end

      def title_section(title, description) do
        Component.section("https://cdn.lara.lv/emoji/sos.webp", [
          Component.text("# #{title}"),
          Component.text(description)
        ])
      end

      def footer_section do
        Component.text("-# Use the buttons below for more information.")
      end

      def action_row(homepage) do
        ActionRow.action_row([
          Button.link_button("Homepage", homepage,
            emoji: %Nostrum.Struct.Emoji{
              id: 1_396_299_330_457_178_293,
              name: "globe_showing_europe_africa",
              animated: true
            }
          ),
          Button.link_button("Support Server", "https://discord.com/invite/KUMdnjcE97",
            emoji: %Nostrum.Struct.Emoji{
              id: 1_396_297_056_750_014_546,
              name: "wave",
              animated: true
            }
          )
        ])
      end
    end
  end
end
