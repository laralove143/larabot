use anyhow::Result;
use sparkle_interactions::InteractionHandle;
use twilight_model::application::{
    command::{Command, CommandType},
    interaction::Interaction,
};
use twilight_util::builder::command::CommandBuilder;

use crate::{
    interaction::{
        feedback::modal::FeedbackModal, AppInteraction, CreateAppInteraction, CreateCommand,
        CreateModal,
    },
    localization::{text_lang_kv, Locale},
    text,
};

pub struct FeedbackCommand {
    locale: Locale,
}

impl CreateCommand for FeedbackCommand {
    fn command() -> Result<Command> {
        Ok(CommandBuilder::new(
            Self::CUSTOM_ID,
            "Provide feedback for the bot",
            CommandType::ChatInput,
        )
        .name_localizations(text_lang_kv(text::feedback_command::name)?)
        .description_localizations(text_lang_kv(text::feedback_command::description)?)
        .build())
    }
}

impl CreateAppInteraction for FeedbackCommand {
    fn new(interaction: Interaction) -> Result<Self> {
        Ok(Self {
            locale: interaction.locale.into(),
        })
    }
}

impl AppInteraction for FeedbackCommand {
    const CUSTOM_ID: &'static str = "feedback";
    const IS_EPHEMERAL: bool = true;

    async fn run(self, handle: InteractionHandle) -> Result<()> {
        handle
            .respond(FeedbackModal::show_response(&self.locale)?)
            .await?;

        Ok(())
    }
}
