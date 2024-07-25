use anyhow::Result;
use sparkle_interactions::InteractionHandle;
use twilight_model::application::{
    command::{Command, CommandType},
    interaction::Interaction,
};
use twilight_util::builder::command::CommandBuilder;

use crate::{
    interaction::{
        feedback::modal::{FeedbackModal, FeedbackModalRequiredData},
        AppInteraction, CreateCommand, CreateModal,
    },
    localization::LocalizedText,
};

// noinspection SpellCheckingInspection
const NAME: LocalizedText = LocalizedText {
    bulgarian: "Оставете обратна връзка за бота",
    chinese_cn: "为机器人提供反馈",
    chinese_tw: "為機器人提供反饋",
    croatian: "Pružite povratne informacije za bota",
    czech: "Poskytněte zpětnou vazbu pro bota",
    danish: "Giv feedback til botten",
    dutch: "Geef feedback voor de bot",
    english_uk: "Provide feedback for the bot",
    english_us: "Provide feedback for the bot",
    finnish: "Anna palautetta botista",
    french: "Fournissez des retours sur le bot",
    german: "Geben Sie Feedback zum Bot",
    greek: "Παρέχετε σχόλια για το bot",
    hindi: "बॉट के लिए प्रतिक्रिया प्रदान करें",
    hungarian: "Adjon visszajelzést a botról",
    indonesian: "Berikan umpan balik untuk bot",
    italian: "Fornisci feedback per il bot",
    japanese: "ボットにフィードバックを提供する",
    korean: "봇에 대한 피드백 제공",
    lithuanian: "Pateikite atsiliepimus apie botą",
    norwegian: "Gi tilbakemelding på botten",
    polish: "Zapewnij opinię o bocie",
    portuguese_br: "Forneça feedback para o bot",
    romanian: "Oferiți feedback pentru bot",
    russian: "Оставьте отзыв о боте",
    spanish: "Proporcione retroalimentación para el bot",
    spanish_latam: "Proporcione retroalimentación para el bot",
    swedish: "Ge feedback på botten",
    thai: "ให้ข้อเสนอแนะสำหรับบอท",
    turkish: "Bot için geri bildirim sağlayın",
    ukrainian: "Надайте відгук про бота",
    vietnamese: "Cung cấp phản hồi cho bot",
};

// noinspection SpellCheckingInspection
const DESCRIPTION: LocalizedText = LocalizedText {
    bulgarian: "обратна_връзка",
    chinese_cn: "反馈",
    chinese_tw: "反饋",
    croatian: "povratna_informacija",
    czech: "zpětná_vazba",
    danish: "feedback",
    dutch: "feedback",
    english_uk: "feedback",
    english_us: "feedback",
    finnish: "palaute",
    french: "retour",
    german: "feedback",
    greek: "σχόλια",
    hindi: "प्रतिक्रिया",
    hungarian: "visszajelzés",
    indonesian: "umpan_balik",
    italian: "feedback",
    japanese: "フィードバック",
    korean: "피드백",
    lithuanian: "atsiliepimas",
    norwegian: "tilbakemelding",
    polish: "opinia",
    portuguese_br: "feedback",
    romanian: "feedback",
    russian: "обратная_связь",
    spanish: "retroalimentación",
    spanish_latam: "feedback",
    swedish: "feedback",
    thai: "ข้อเสนอแนะ",
    turkish: "geri_bildirim",
    ukrainian: "відгук",
    vietnamese: "phản_hồi",
};

pub struct FeedbackCommand {
    locale: Option<String>,
}

impl CreateCommand for FeedbackCommand {
    fn command() -> Result<Command> {
        Ok(CommandBuilder::new(
            Self::CUSTOM_ID,
            "Provide feedback for the bot",
            CommandType::ChatInput,
        )
        .name_localizations(NAME.as_discord_localized_kv())
        .description_localizations(DESCRIPTION.as_discord_localized_kv())
        .build())
    }
}

impl AppInteraction for FeedbackCommand {
    type RequiredData = ();

    const CUSTOM_ID: &'static str = "feedback";
    const IS_EPHEMERAL: bool = true;

    fn new(interaction: Interaction, _data: Self::RequiredData) -> Result<Self> {
        Ok(Self {
            locale: interaction.locale,
        })
    }

    async fn run(self, handle: InteractionHandle) -> Result<()> {
        handle
            .respond(FeedbackModal::show_response(FeedbackModalRequiredData {
                locale: self.locale,
            })?)
            .await?;

        Ok(())
    }
}
