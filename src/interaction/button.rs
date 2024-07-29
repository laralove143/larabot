use anyhow::Result;
use sparkle_interactions::builder::component::ButtonBuilder;
use twilight_model::{channel::message::component::Button, id::Id};

use crate::{localization::Locale, text};

pub fn support_server_invite(locale: &Locale) -> Result<Button> {
    Ok(ButtonBuilder::with_url(
        "https://discord.com/invite/KUMdnjcE97".to_owned(),
        text::support_server_button::label(locale)?.to_string(),
    )
    .custom_emoji(
        "wave".to_owned(),
        #[allow(clippy::unreadable_literal)]
        Id::new(1266148304048095282),
        true,
    )
    .build())
}
