use anyhow::Result;
use fluent_static::{fluent_bundle::FluentError, Message};
use tracing::warn;

#[derive(Debug)]
pub struct Locale(Option<String>);

impl<T: Into<String>> From<Option<T>> for Locale {
    fn from(locale: Option<T>) -> Self {
        Self(locale.map(Into::into))
    }
}

impl AsRef<str> for Locale {
    fn as_ref(&self) -> &str {
        self.0.as_deref().unwrap_or("en-US")
    }
}

pub fn text_lang_kv<'a, F: Fn(&'a str) -> Result<Message<'a>, FluentError>>(
    message: F,
) -> Result<[(&'static str, String); 32]> {
    let default_text = message("en-US")?;

    Ok([
        "bg", "zh-CN", "zh-TW", "hr", "cs", "da", "nl", "en-GB", "en-US", "fi", "fr", "de", "el",
        "hi", "hu", "id", "it", "ja", "ko", "lt", "no", "pl", "pt-BR", "ro", "ru", "es-419",
        "es-ES", "sv-SE", "th", "tr", "uk", "vi",
    ]
    .map(|lang_code| {
        let text = message(lang_code)
            .unwrap_or_else(|err| {
                warn!(
                    error = ?err,
                    "Failed to get localized text, defaulting to English."
                );

                default_text.clone()
            })
            .to_string();

        (lang_code, text)
    }))
}
