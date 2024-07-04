use sparkle_interactions::builder::{
    component::{ButtonBuilder, ComponentsBuilder},
    InteractionResponseBuilder,
};
use twilight_model::{http::interaction::InteractionResponse, id::Id};
use twilight_util::builder::{
    embed::{EmbedBuilder, EmbedFooterBuilder, ImageSource},
    InteractionResponseDataBuilder,
};

use crate::{color::Color, localization::LocalizedText};

// noinspection SpellCheckingInspection
const EMBED_TITLE: LocalizedText = LocalizedText {
    bulgarian: "Нещо се обърка...",
    chinese_cn: "出问题了...",
    chinese_tw: "出了問題...",
    croatian: "Nešto je pošlo po zlu...",
    czech: "Něco se pokazilo...",
    danish: "Noget gik galt...",
    dutch: "Er is iets fout gegaan...",
    english_uk: "Something went wrong...",
    english_us: "Something went wrong...",
    finnish: "Jotain meni pieleen...",
    french: "Quelque chose a mal tourné...",
    german: "Etwas ist schiefgelaufen...",
    greek: "Κάτι πήγε στραβά...",
    hindi: "कुछ गलत हो गया...",
    hungarian: "Valami elromlott...",
    indonesian: "Ada yang salah...",
    italian: "Qualcosa è andato storto...",
    japanese: "何かがうまくいかなかった...",
    korean: "문제가 발생했습니다...",
    lithuanian: "Kažkas nutiko...",
    norwegian: "Noe gikk galt...",
    polish: "Coś poszło nie tak...",
    portuguese_br: "Algo deu errado...",
    romanian: "Ceva a mers prost...",
    russian: "Что-то пошло не так...",
    spanish: "Algo salió mal...",
    spanish_latam: "Algo salió mal...",
    swedish: "Något gick fel...",
    thai: "เกิดข้อผิดพลาด...",
    turkish: "Bir şeyler ters gitti...",
    ukrainian: "Щось пішло не так...",
    vietnamese: "Đã xảy ra lỗi...",
};

// noinspection SpellCheckingInspection
#[rustfmt::skip]
const EMBED_DESCRIPTION: LocalizedText = LocalizedText {
    bulgarian: "Възникна грешка при обработката на вашата команда. Грешката беше съобщена на разработчика. Благодарим за търпението.",
    chinese_cn: "尝试处理您的命令时发生错误。该错误已报告给开发者。感谢您的耐心。",
    chinese_tw: "嘗試處理您的命令時發生錯誤。該錯誤已報告給開發者。感謝您的耐心。",
    croatian: "Došlo je do pogreške prilikom pokušaja obrade vaše naredbe. Pogreška je prijavljena razvojnom programeru. Hvala na strpljenju.",
    czech: "Při zpracování vašeho příkazu došlo k chybě. Chyba byla nahlášena vývojáři. Děkujeme za vaši trpělivost.",
    danish: "Der opstod en fejl under forsøget på at behandle din kommando. Fejlen er rapporteret til udvikleren. Tak for din tålmodighed.",
    dutch: "Er is een fout opgetreden bij het verwerken van uw opdracht. De fout is gemeld aan de ontwikkelaar. Bedankt voor uw geduld.",
    english_uk: "An error occurred while trying to process your command. The error has been reported to the developer. Thank you for your patience.",
    english_us: "An error occurred while trying to process your command. The error has been reported to the developer. Thank you for your patience.",
    finnish: "Komennon käsittelyssä tapahtui virhe. Virhe on raportoitu kehittäjälle. Kiitos kärsivällisyydestäsi.",
    french: "Une erreur s'est produite lors du traitement de votre commande. L'erreur a été signalée au développeur. Merci de votre patience.",
    german: "Beim Versuch, Ihren Befehl zu verarbeiten, ist ein Fehler aufgetreten. Der Fehler wurde dem Entwickler gemeldet. Vielen Dank für Ihre Geduld.",
    greek: "Παρουσιάστηκε σφάλμα κατά την επεξεργασία της εντολής σας. Το σφάλμα έχει αναφερθεί στον προγραμματιστή. Ευχαριστούμε για την υπομονή σας.",
    hindi: "आपके कमांड को प्रोसेस करने की कोशिश में एक त्रुटि हुई। त्रुटि को डेवलपर को सूचित कर दिया गया है। धन्यवाद आपके धैर्य के लिए।",
    hungarian: "Hiba történt a parancs feldolgozása közben. A hibát jelentették a fejlesztőnek. Köszönjük türelmét.",
    indonesian: "Terjadi kesalahan saat mencoba memproses perintah Anda. Kesalahan telah dilaporkan kepada pengembang. Terima kasih atas kesabaran Anda.",
    italian: "Si è verificato un errore durante il tentativo di elaborare il tuo comando. L'errore è stato segnalato allo sviluppatore. Grazie per la tua pazienza.",
    japanese: "コマンドの処理中にエラーが発生しました。エラーは開発者に報告されました。ご協力ありがとうございます。",
    korean: "명령을 처리하는 동안 오류가 발생했습니다. 오류는 개발자에게 보고되었습니다. 인내해 주셔서 감사합니다.",
    lithuanian: "Bando apdoroti jūsų komandą įvyko klaida. Klaida pranešta kūrėjui. Ačiū už jūsų kantrybę.",
    norwegian: "Det oppstod en feil under forsøket på å behandle kommandoen din. Feilen er rapportert til utvikleren. Takk for din tålmodighet.",
    polish: "Wystąpił błąd podczas próby przetworzenia twojej komendy. Błąd został zgłoszony do dewelopera. Dziękujemy za cierpliwość.",
    portuguese_br: "Ocorreu um erro ao tentar processar seu comando. O erro foi reportado ao desenvolvedor. Obrigado pela sua paciência.",
    romanian: "A apărut o eroare în timpul încercării de a procesa comanda dumneavoastră. Eroarea a fost raportată dezvoltatorului. Vă mulțumim pentru răbdare.",
    russian: "Произошла ошибка при попытке обработать вашу команду. Ошибка была сообщена разработчику. Спасибо за терпение.",
    spanish: "Ocurrió un error al intentar procesar tu comando. El error ha sido reportado al desarrollador. Gracias por tu paciencia.",
    spanish_latam: "Ocurrió un error al intentar procesar tu comando. El error ha sido reportado al desarrollador. Gracias por tu paciencia.",
    swedish: "Ett fel inträffade när ditt kommando försökte bearbetas. Felet har rapporterats till utvecklaren. Tack för ditt tålamod.",
    thai: "เกิดข้อผิดพลาดขณะพยายามดำเนินการคำสั่งของคุณ ข้อผิดพลาดได้ถูกรายงานไปยังนักพัฒนา ขอบคุณสำหรับความอดทนของคุณ",
    turkish: "Komutunuz işlenirken bir hata oluştu. Hata geliştiriciye bildirildi. Sabrınız için teşekkür ederiz.",
    ukrainian: "Під час спроби обробити вашу команду сталася помилка. Помилка була повідомлена розробнику. Дякуємо за ваше терпіння.",
    vietnamese: "Đã xảy ra lỗi khi cố gắng xử lý lệnh của bạn. Lỗi đã được báo cáo cho nhà phát triển. Cảm ơn bạn đã kiên nhẫn.",
};

// noinspection SpellCheckingInspection
#[rustfmt::skip]
const EMBED_FOOTER_TEXT: LocalizedText = LocalizedText {
    bulgarian: "Не се колебайте да се свържете с разработчика от сървъра за поддръжка за повече информация.",
    chinese_cn: "如需更多信息，请随时联系支持服务器上的开发者。",
    chinese_tw: "如需更多資訊，請隨時聯繫支持伺服器上的開發者。",
    croatian: "Slobodno kontaktirajte razvojnog programera sa poslužitelja podrške za više informacija.",
    czech: "Pro další informace neváhejte kontaktovat vývojáře na serveru podpory.",
    danish: "Du er velkommen til at kontakte udvikleren fra supportserveren for yderligere oplysninger.",
    dutch: "Neem gerust contact op met de ontwikkelaar van de ondersteuningsserver voor meer informatie.",
    english_uk: "Feel free to contact the developer from the support server for more information.",
    english_us: "Feel free to contact the developer from the support server for more information.",
    finnish: "Voit ottaa yhteyttä kehittäjään tukipalvelimelta lisätietoja varten.",
    french: "N'hésitez pas à contacter le développeur via le serveur de support pour plus d'informations.",
    german: "Kontaktieren Sie den Entwickler vom Support-Server aus für weitere Informationen.",
    greek: "Μη διστάσετε να επικοινωνήσετε με τον προγραμματιστή από τον διακομιστή υποστήριξης για περισσότερες πληροφορίες.",
    hindi: "अधिक जानकारी के लिए सहायता सर्वर से डेवलपर से संपर्क करने में संकोच न करें।",
    hungarian: "Nyugodtan lépjen kapcsolatba a fejlesztővel a támogatási szerveren keresztül további információkért.",
    indonesian: "Jangan ragu untuk menghubungi pengembang dari server dukungan untuk informasi lebih lanjut.",
    italian: "Sentiti libero di contattare lo sviluppatore dal server di supporto per maggiori informazioni.",
    japanese: "詳細については、サポートサーバーの開発者にお気軽にお問い合わせください。",
    korean: "추가 정보가 필요하시면 지원 서버의 개발자에게 언제든지 연락하십시오.",
    lithuanian: "Jei reikia daugiau informacijos, nedvejodami susisiekite su kūrėju palaikymo serveryje.",
    norwegian: "Du kan gjerne kontakte utvikleren fra støtteserveren for mer informasjon.",
    polish: "Skontaktuj się z deweloperem z serwera wsparcia, jeśli potrzebujesz więcej informacji.",
    portuguese_br: "Sinta-se à vontade para contatar o desenvolvedor do servidor de suporte para mais informações.",
    romanian: "Nu ezitați să contactați dezvoltatorul de pe serverul de suport pentru mai multe informații.",
    russian: "Не стесняйтесь обращаться к разработчику с сервера поддержки за дополнительной информацией.",
    spanish: "No dude en contactar al desarrollador del servidor de soporte para obtener más información.",
    spanish_latam: "No dude en contactar al desarrollador del servidor de soporte para obtener más información.",
    swedish: "Du är välkommen att kontakta utvecklaren från supportservern för mer information.",
    thai: "อย่าลังเลที่จะติดต่อนักพัฒนาจากเซิร์ฟเวอร์สนับสนุนเพื่อขอข้อมูลเพิ่มเติม",
    turkish: "Daha fazla bilgi için destek sunucusundaki geliştiriciyle iletişime geçmekten çekinmeyin.",
    ukrainian: "Не соромтеся звертатися до розробника з сервера підтримки за додатковою інформацією.",
    vietnamese: "Đừng ngại liên hệ với nhà phát triển từ máy chủ hỗ trợ để biết thêm thông tin.",
};

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

pub fn error_response(locale: Option<&str>) -> InteractionResponse {
    let error_embed = EmbedBuilder::new()
        .color(Color::Red.into())
        .thumbnail(ImageSource::url("https://cdn.lara.lv/emoji%2Fgrimacing.gif").unwrap())
        .title(EMBED_TITLE.get_with_default(locale))
        .description(EMBED_DESCRIPTION.get_with_default(locale))
        .footer(EmbedFooterBuilder::new(
            EMBED_FOOTER_TEXT.get_with_default(locale),
        ))
        .build();

    let error_components = ComponentsBuilder::new()
        .buttons(vec![
            ButtonBuilder::with_url(
                "https://discord.com/invite/KUMdnjcE97".to_owned(),
                SUPPORT_SERVER_BUTTON_LABEL
                    .get_with_default(locale)
                    .to_owned(),
            )
            .custom_emoji(
                "bellhopbell".to_owned(),
                #[allow(clippy::unreadable_literal)]
                Id::new(1251928481474936955),
                true,
            )
            .build(),
        ])
        .build();

    InteractionResponseBuilder::send_message(
        InteractionResponseDataBuilder::new()
            .embeds([error_embed])
            .components(error_components)
            .build(),
    )
}
