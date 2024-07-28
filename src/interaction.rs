use std::{sync::Arc, time::Duration};

use anyhow::Result;
use sparkle_interactions::{
    builder::InteractionResponseBuilder, extract::ExtractInteractionData, InteractionHandle,
};
use tracing::info;
use twilight_http::Client;
use twilight_model::{
    application::{command::Command, interaction::Interaction},
    channel::message::component::{Button, TextInput},
    http::interaction::InteractionResponse,
    id::{
        marker::{ApplicationMarker, GuildMarker},
        Id,
    },
};

use crate::{
    interaction::feedback::{button::FeedbackButton, command::FeedbackCommand},
    option_ext::OptionExt,
};

mod button;
mod error_response;
mod feedback;

pub trait CreateCommand {
    fn command() -> Result<Command>;
}

pub trait CreateButton {
    type RequiredData;

    fn button(data: Self::RequiredData, locale: Option<&str>) -> Result<Button>;
}

pub trait CreateModal {
    type RequiredData;

    fn show_response(data: Self::RequiredData, locale: Option<&str>)
    -> Result<InteractionResponse>;
}

pub trait CreateTextInput {
    type RequiredData;

    const CUSTOM_ID: &'static str;

    fn text_input(data: Self::RequiredData, locale: Option<&str>) -> Result<TextInput>;
}

pub trait AppInteraction: Sized {
    type RequiredData;

    const CUSTOM_ID: &'static str;
    const IS_EPHEMERAL: bool;

    fn new(interaction: Interaction, data: Self::RequiredData) -> Result<Self>;

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
                )?)
                .await?;

            return Err(err);
        }

        Ok(())
    }
}

pub async fn handle_common_interaction(
    client: Arc<Client>,
    interaction: Interaction,
) -> Result<()> {
    let custom_id = interaction.data.as_ref().ok()?.custom_id().ok()?;

    #[allow(unreachable_patterns)]
    match custom_id {
        FeedbackButton::CUSTOM_ID => {
            FeedbackButton::new(interaction.clone(), ())?
                .handle(client, &interaction)
                .await?;
        }
        FeedbackCommand::CUSTOM_ID => {
            FeedbackCommand::new(interaction.clone(), ())?
                .handle(client, &interaction)
                .await?;
        }
        _ => (),
    }

    Ok(())
}

pub async fn set_commands(
    client: &Client,
    application_id: Id<ApplicationMarker>,
    mut commands: Vec<Command>,
    guild_id: Id<GuildMarker>,
) -> Result<()> {
    let interaction_client = client.interaction(application_id);

    commands.push(FeedbackCommand::command()?);

    let command_names = commands
        .iter()
        .map(|c| c.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    if cfg!(debug_assertions) {
        interaction_client
            .set_guild_commands(guild_id, &commands)
            .await?;

        info!(
            Commands = command_names,
            "Guild ID" = guild_id.to_string(),
            "Set commands in testing guild.",
        );
    } else {
        interaction_client.set_global_commands(&commands).await?;

        info!(Commands = command_names, "Set commands globally.");
    }

    Ok(())
}
