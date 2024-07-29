use std::{borrow::ToOwned, sync::Arc};

use anyhow::Result;
use sparkle_interactions::{
    builder::{
        component::{ComponentsBuilder, TextInputBuilder},
        InteractionResponseBuilder,
    },
    extract::{ExtractInteractionData, ExtractModalComponentRef},
    InteractionHandle,
};
use twilight_model::{
    application::interaction::Interaction, channel::message::component::TextInput,
    http::interaction::InteractionResponse,
};
use twilight_util::builder::{
    embed::{EmbedBuilder, EmbedFooterBuilder, ImageSource},
    InteractionResponseDataBuilder,
};

use crate::{
    color::Color,
    interaction::{
        button::support_server_invite, AppInteraction, CreateAppInteractionWithData, CreateModal,
        CreateTextInput,
    },
    localization::Locale,
    option_ext::OptionExt,
    sentry_user_feedback::FeedbackClient,
    text,
};

struct ContactTextInput;

impl CreateTextInput for ContactTextInput {
    const CUSTOM_ID: &'static str = "contact";

    fn text_input(locale: &Locale) -> Result<TextInput> {
        Ok(TextInputBuilder::new(
            text::feedback_command::modal_contact_label(locale)?.to_string(),
            Self::CUSTOM_ID.to_owned(),
        )
        .placeholder(text::feedback_command::modal_contact_placeholder(locale)?.to_string())
        .build())
    }
}

struct ContentTextInput;

impl CreateTextInput for ContentTextInput {
    const CUSTOM_ID: &'static str = "content";

    fn text_input(locale: &Locale) -> Result<TextInput> {
        Ok(TextInputBuilder::new(
            text::feedback_command::modal_content_label(locale)?.to_string(),
            Self::CUSTOM_ID.to_owned(),
        )
        .paragraph()
        .require()
        .build())
    }
}

pub struct FeedbackModal {
    contact: String,
    content: String,
    feedback_client: Arc<FeedbackClient>,
    locale: Locale,
}

impl CreateModal for FeedbackModal {
    fn show_response(locale: &Locale) -> Result<InteractionResponse> {
        Ok(InteractionResponseBuilder::show_modal(
            text::feedback_command::modal_title(locale)?.to_string(),
            Self::CUSTOM_ID.to_owned(),
        )
        .text_input(ContentTextInput::text_input(locale)?)
        .text_input(ContactTextInput::text_input(locale)?)
        .build())
    }
}

impl CreateAppInteractionWithData for FeedbackModal {
    type Data = Arc<FeedbackClient>;

    fn new(interaction: Interaction, data: Self::Data) -> Result<Self> {
        let components = interaction.data.ok()?.modal_data().ok()?.components;

        Ok(Self {
            contact: components
                .component(ContactTextInput::CUSTOM_ID)
                .ok()?
                .to_owned(),
            content: components
                .component(ContentTextInput::CUSTOM_ID)
                .ok()?
                .to_owned(),
            feedback_client: data,
            locale: interaction.locale.into(),
        })
    }
}

impl AppInteraction for FeedbackModal {
    const CUSTOM_ID: &'static str = "feedback_form";
    const IS_EPHEMERAL: bool = true;

    async fn run(self, handle: InteractionHandle) -> Result<()> {
        self.feedback_client
            .create_user_feedback(
                self.contact
                    .is_empty()
                    .then(|| "Empty".to_owned())
                    .unwrap_or(self.contact),
                self.content,
            )
            .await?;

        let embed = EmbedBuilder::new()
            .color(Color::Green.into())
            .thumbnail(ImageSource::url("https://cdn.lara.lv/emoji/pencil.gif")?)
            .title(text::feedback_command::embed_title(&self.locale)?.to_string())
            .description(text::feedback_command::embed_description(&self.locale)?.to_string())
            .footer(EmbedFooterBuilder::new(
                text::feedback_command::embed_footer(&self.locale)?.to_string(),
            ))
            .build();

        handle
            .respond(InteractionResponseBuilder::send_message(
                InteractionResponseDataBuilder::new()
                    .embeds([embed])
                    .components(
                        ComponentsBuilder::new()
                            .buttons(vec![support_server_invite(&self.locale)?])
                            .build(),
                    )
                    .build(),
            ))
            .await?;

        Ok(())
    }
}
