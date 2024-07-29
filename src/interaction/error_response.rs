use anyhow::Result;
use sparkle_interactions::builder::{component::ComponentsBuilder, InteractionResponseBuilder};
use twilight_model::http::interaction::InteractionResponse;
use twilight_util::builder::{
    embed::{EmbedBuilder, EmbedFooterBuilder, ImageSource},
    InteractionResponseDataBuilder,
};

use crate::{
    color::Color,
    interaction::{button::support_server_invite, feedback::button::FeedbackButton, CreateButton},
    localization::Locale,
    text,
};

pub fn error_response(locale: &Locale) -> Result<InteractionResponse> {
    let error_embed = EmbedBuilder::new()
        .color(Color::Red.into())
        .thumbnail(ImageSource::url("https://cdn.lara.lv/emoji/grimacing.gif").unwrap())
        .title(text::error_response::title(locale)?.to_string())
        .description(text::error_response::description(locale)?.to_string())
        .footer(EmbedFooterBuilder::new(
            text::error_response::footer(locale)?.to_string(),
        ))
        .build();

    Ok(InteractionResponseBuilder::send_message(
        InteractionResponseDataBuilder::new()
            .embeds([error_embed])
            .components(
                ComponentsBuilder::new()
                    .buttons(vec![
                        FeedbackButton::button(locale)?,
                        support_server_invite(locale)?,
                    ])
                    .build(),
            )
            .build(),
    ))
}
