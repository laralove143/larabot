use tracing::warn;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalizedText {
    pub bulgarian: &'static str,
    pub chinese_cn: &'static str,
    pub chinese_tw: &'static str,
    pub croatian: &'static str,
    pub czech: &'static str,
    pub danish: &'static str,
    pub dutch: &'static str,
    pub english_uk: &'static str,
    pub english_us: &'static str,
    pub finnish: &'static str,
    pub french: &'static str,
    pub german: &'static str,
    pub greek: &'static str,
    pub hindi: &'static str,
    pub hungarian: &'static str,
    pub indonesian: &'static str,
    pub italian: &'static str,
    pub japanese: &'static str,
    pub korean: &'static str,
    pub lithuanian: &'static str,
    pub norwegian: &'static str,
    pub polish: &'static str,
    pub portuguese_br: &'static str,
    pub romanian: &'static str,
    pub russian: &'static str,
    pub spanish: &'static str,
    pub spanish_latam: &'static str,
    pub swedish: &'static str,
    pub thai: &'static str,
    pub turkish: &'static str,
    pub ukrainian: &'static str,
    pub vietnamese: &'static str,
}

impl LocalizedText {
    #[must_use]
    pub const fn as_discord_localized_kv(self) -> [(&'static str, &'static str); 32] {
        [
            ("bg", self.bulgarian),
            ("zh-CN", self.chinese_cn),
            ("zh-TW", self.chinese_tw),
            ("hr", self.croatian),
            ("cs", self.czech),
            ("da", self.danish),
            ("nl", self.dutch),
            ("en-GB", self.english_uk),
            ("en-US", self.english_us),
            ("fi", self.finnish),
            ("fr", self.french),
            ("de", self.german),
            ("el", self.greek),
            ("hi", self.hindi),
            ("hu", self.hungarian),
            ("id", self.indonesian),
            ("it", self.italian),
            ("ja", self.japanese),
            ("ko", self.korean),
            ("lt", self.lithuanian),
            ("no", self.norwegian),
            ("pl", self.polish),
            ("pt-BR", self.portuguese_br),
            ("ro", self.romanian),
            ("ru", self.russian),
            ("es-419", self.spanish),
            ("es-ES", self.spanish_latam),
            ("sv-SE", self.swedish),
            ("th", self.thai),
            ("tr", self.turkish),
            ("uk", self.ukrainian),
            ("vi", self.vietnamese),
        ]
    }

    #[must_use]
    pub fn get(self, locale: Option<&str>) -> &str {
        locale
            .and_then(|loc| self.get_with_locale(loc))
            .unwrap_or_else(|| {
                warn!(
                    language_code = locale.unwrap_or_default(),
                    "Unknown locale, defaulting to English."
                );

                self.english_us
            })
    }

    fn get_with_locale(self, locale: &str) -> Option<&str> {
        Some(match locale {
            "bg" => self.bulgarian,
            "zh-CN" => self.chinese_cn,
            "zh-TW" => self.chinese_tw,
            "hr" => self.croatian,
            "cs" => self.czech,
            "da" => self.danish,
            "nl" => self.dutch,
            "en-GB" => self.english_uk,
            "en-US" => self.english_us,
            "fi" => self.finnish,
            "fr" => self.french,
            "de" => self.german,
            "el" => self.greek,
            "hi" => self.hindi,
            "hu" => self.hungarian,
            "id" => self.indonesian,
            "it" => self.italian,
            "ja" => self.japanese,
            "ko" => self.korean,
            "lt" => self.lithuanian,
            "no" => self.norwegian,
            "pl" => self.polish,
            "pt-BR" => self.portuguese_br,
            "ro" => self.romanian,
            "ru" => self.russian,
            "es-419" => self.spanish,
            "es-ES" => self.spanish_latam,
            "sv-SE" => self.swedish,
            "th" => self.thai,
            "tr" => self.turkish,
            "uk" => self.ukrainian,
            "vi" => self.vietnamese,
            _ => return None,
        })
    }
}
