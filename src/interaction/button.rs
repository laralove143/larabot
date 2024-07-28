use sparkle_interactions::builder::component::ButtonBuilder;
use twilight_model::{channel::message::component::Button, id::Id};

use crate::localization::LocalizedText;

// noinspection SpellCheckingInspection
const SUPPORT_SERVER_BUTTON_LABEL: LocalizedText = LocalizedText {
    bulgarian: "Сървър за поддръжка",
    chinese_cn: "支持服务器",
    chinese_tw: "支援伺服器",
    croatian: "Poslužitelj podrške",
    czech: "Podpůrný server",
    danish: "Supportserver",
    dutch: "Supportserver",
    english_uk: "Support Server",
    english_us: "Support Server",
    finnish: "Tukipalvelin",
    french: "Serveur de support",
    german: "Support-Server",
    greek: "Διακομιστής υποστήριξης",
    hindi: "समर्थन सर्वर",
    hungarian: "Támogatási szerver",
    indonesian: "Server Dukungan",
    italian: "Server di supporto",
    japanese: "サポートサーバー",
    korean: "지원 서버",
    lithuanian: "Palaikymo serveris",
    norwegian: "Supportserver",
    polish: "Serwer wsparcia",
    portuguese_br: "Servidor de Suporte",
    romanian: "Server de suport",
    russian: "Сервер поддержки",
    spanish: "Servidor de Soporte",
    spanish_latam: "Servidor de Soporte",
    swedish: "Supportserver",
    thai: "เซิร์ฟเวอร์สนับสนุน",
    turkish: "Destek Sunucusu",
    ukrainian: "Сервер підтримки",
    vietnamese: "Máy chủ hỗ trợ",
};

pub fn support_server_invite(locale: Option<&str>) -> Button {
    ButtonBuilder::with_url(
        "https://discord.com/invite/KUMdnjcE97".to_owned(),
        SUPPORT_SERVER_BUTTON_LABEL.get(locale).to_owned(),
    )
    .custom_emoji(
        "wave".to_owned(),
        #[allow(clippy::unreadable_literal)]
        Id::new(1266148304048095282),
        true,
    )
    .build()
}
