use std::{sync::Arc, time::Duration};

use anyhow::Result;
use sparkle_interactions::{builder::InteractionResponseBuilder, InteractionHandle};
use tracing::info;
use twilight_http::Client;
use twilight_model::{
    application::{command::Command, interaction::Interaction},
    id::{
        marker::{ApplicationMarker, GuildMarker},
        Id,
    },
};

mod error_response;

pub trait CreateCommand {
    fn command() -> Result<Command>;
}

pub trait AppInteraction: Sized {
    const CUSTOM_ID: &'static str;
    const IS_EPHEMERAL: bool;

    fn new(interaction: Interaction) -> Result<Self>;

    async fn run(self, handle: InteractionHandle) -> Result<()>;

    async fn handle(self, client: Arc<Client>, interaction: &Interaction) -> Result<()> {
        let handle = InteractionHandle::new(
            client,
            interaction.application_id,
            interaction.id,
            interaction.token.clone(),
        )
        .track_last_message()
        .respond_on_delay(
            {
                let mut response = InteractionResponseBuilder::defer_send_message();

                if Self::IS_EPHEMERAL {
                    response = response.ephemeral();
                }

                response.build()
            },
            Duration::from_secs(2),
        );

        if let Err(err) = self.run(handle.clone()).await {
            handle
                .respond(error_response::error_response(
                    interaction.locale.as_deref(),
                ))
                .await?;

            return Err(err);
        }

        Ok(())
    }
}

pub async fn set_commands(
    client: &Client,
    application_id: Id<ApplicationMarker>,
    commands: &[Command],
    guild_id: Id<GuildMarker>,
) -> Result<()> {
    let interaction_client = client.interaction(application_id);

    let command_names = commands
        .iter()
        .map(|c| c.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    if cfg!(debug_assertions) {
        interaction_client
            .set_guild_commands(guild_id, commands)
            .await?;

        info!(
            Commands = command_names,
            "Guild ID" = guild_id.to_string(),
            "Set commands in testing guild.",
        );
    } else {
        interaction_client.set_global_commands(commands).await?;

        info!(Commands = command_names, "Set commands globally.");
    }

    Ok(())
}
