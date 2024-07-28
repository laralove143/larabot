use anyhow::Result;
use sparkle_interactions::{builder::component::ButtonBuilder, InteractionHandle};
use twilight_model::{
    application::interaction::Interaction,
    channel::message::component::{Button, ButtonStyle},
};

use crate::{
    interaction::{feedback::modal::FeedbackModal, AppInteraction, CreateButton, CreateModal},
    localization::LocalizedText,
};

// noinspection SpellCheckingInspection
const BUTTON_LABEL: LocalizedText = LocalizedText {
    bulgarian: "Дайте Обратна Връзка",
    chinese_cn: "提供反馈",
    chinese_tw: "提供回饋",
    croatian: "Pruži Povratne Informacije",
    czech: "Poskytnout Zpětnou Vazbu",
    danish: "Giv Feedback",
    dutch: "Geef Feedback",
    english_uk: "Provide Feedback",
    english_us: "Provide Feedback",
    finnish: "Anna Palaute",
    french: "Donnez Votre Avis",
    german: "Feedback Geben",
    greek: "Δώστε Ανατροφοδότηση",
    hindi: "प्रतिक्रिया दें",
    hungarian: "Visszajelzés Adása",
    indonesian: "Berikan Umpan Balik",
    italian: "Fornisci Feedback",
    japanese: "フィードバックを提供する",
    korean: "피드백 제공",
    lithuanian: "Pateikti Atsiliepimą",
    norwegian: "Gi Tilbakemelding",
    polish: "Zapewnij Opinię",
    portuguese_br: "Fornecer Feedback",
    romanian: "Oferiți Feedback",
    russian: "Оставить Отзыв",
    spanish: "Proporcionar Retroalimentación",
    spanish_latam: "Proporcionar Retroalimentación",
    swedish: "Ge Feedback",
    thai: "ให้ข้อเสนอแนะ",
    turkish: "Geri Bildirim Ver",
    ukrainian: "Надати Відгук",
    vietnamese: "Cung Cấp Phản Hồi",
};

pub struct FeedbackButton {
    locale: Option<String>,
}

impl CreateButton for FeedbackButton {
    type RequiredData = ();

    fn button(_data: Self::RequiredData, locale: Option<&str>) -> Result<Button> {
        Ok(ButtonBuilder::with_custom_id(
            Self::CUSTOM_ID.to_owned(),
            BUTTON_LABEL.get_with_default(locale).to_owned(),
            ButtonStyle::Secondary,
        )
        .build())
    }
}

impl AppInteraction for FeedbackButton {
    type RequiredData = ();

    const CUSTOM_ID: &'static str = "feedback";
    const IS_EPHEMERAL: bool = false;

    fn new(interaction: Interaction, _data: Self::RequiredData) -> Result<Self> {
        Ok(Self {
            locale: interaction.locale,
        })
    }

    async fn run(self, handle: InteractionHandle) -> Result<()> {
        handle
            .respond(FeedbackModal::show_response((), self.locale.as_deref())?)
            .await?;

        Ok(())
    }
}
