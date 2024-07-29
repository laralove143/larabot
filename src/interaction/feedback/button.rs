use anyhow::Result;
use sparkle_interactions::{builder::component::ButtonBuilder, InteractionHandle};
use twilight_model::{
    application::interaction::Interaction,
    channel::message::component::{Button, ButtonStyle},
};

use crate::{
    interaction::{
        feedback::modal::FeedbackModal, AppInteraction, CreateAppInteraction, CreateButton,
        CreateModal,
    },
    localization::Locale,
    text,
};

pub struct FeedbackButton {
    locale: Locale,
}

impl CreateButton for FeedbackButton {
    fn button(locale: &Locale) -> Result<Button> {
        Ok(ButtonBuilder::with_custom_id(
            Self::CUSTOM_ID.to_owned(),
            text::feedback_command::button_label(locale)?.to_string(),
            ButtonStyle::Secondary,
        )
        .build())
    }
}

impl CreateAppInteraction for FeedbackButton {
    fn new(interaction: Interaction) -> Result<Self> {
        Ok(Self {
            locale: interaction.locale.into(),
        })
    }
}

impl AppInteraction for FeedbackButton {
    const CUSTOM_ID: &'static str = "feedback";
    const IS_EPHEMERAL: bool = false;

    async fn run(self, handle: InteractionHandle) -> Result<()> {
        handle
            .respond(FeedbackModal::show_response(&self.locale)?)
            .await?;

        Ok(())
    }
}
