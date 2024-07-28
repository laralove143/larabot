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
    localization::LocalizedText,
};

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
    bulgarian: "Не се колебайте да предоставите повече подробности за грешката, използвайки бутона за обратна връзка.",
    chinese_cn: "随时使用反馈按钮提供有关错误的更多详细信息。",
    chinese_tw: "隨時使用回饋按鈕提供有關錯誤的更多詳細信息。",
    croatian: "Slobodno pružite više detalja o pogrešci koristeći gumb za povratne informacije.",
    czech: "Klidně poskytněte více podrobností o chybě pomocí tlačítka pro zpětnou vazbu.",
    danish: "Du er velkommen til at give flere detaljer om fejlen ved at bruge feedbackknappen.",
    dutch: "Voel je vrij om meer details over de fout te verstrekken met de feedbackknop.",
    english_uk: "Feel free to provide more details about the error using the feedback button.",
    english_us: "Feel free to provide more details about the error using the feedback button.",
    finnish: "Voit antaa lisätietoja virheestä käyttämällä palautepainiketta.",
    french: "N'hésitez pas à fournir plus de détails sur l'erreur en utilisant le bouton de feedback.",
    german: "Fühlen Sie sich frei, mehr Details über den Fehler mit dem Feedback-Button zu geben.",
    greek: "Μη διστάσετε να δώσετε περισσότερες λεπτομέρειες για το σφάλμα χρησιμοποιώντας το κουμπί ανατροφοδότησης.",
    hindi: "फीडबैक बटन का उपयोग करके त्रुटि के बारे में अधिक विवरण प्रदान करने में संकोच न करें।",
    hungarian: "Nyugodtan adjon meg több részletet a hiba kapcsán a visszajelzés gomb használatával.",
    indonesian: "Jangan ragu untuk memberikan lebih banyak detail tentang kesalahan menggunakan tombol umpan balik.",
    italian: "Sentiti libero di fornire ulteriori dettagli sull'errore utilizzando il pulsante di feedback.",
    japanese: "フィードバックボタンを使用してエラーの詳細を自由に提供してください。",
    korean: "피드백 버튼을 사용하여 오류에 대한 자세한 내용을 자유롭게 제공하십시오.",
    lithuanian: "Nedvejodami pateikite daugiau informacijos apie klaidą naudodami atsiliepimų mygtuką.",
    norwegian: "Føl deg fri til å gi flere detaljer om feilen ved å bruke tilbakemeldingsknappen.",
    polish: "Śmiało podaj więcej szczegółów na temat błędu za pomocą przycisku opinii.",
    portuguese_br: "Fique à vontade para fornecer mais detalhes sobre o erro usando o botão de feedback.",
    romanian: "Nu ezitați să oferiți mai multe detalii despre eroare utilizând butonul de feedback.",
    russian: "Не стесняйтесь предоставлять дополнительные сведения об ошибке, используя кнопку отзыва.",
    spanish: "No dude en proporcionar más detalles sobre el error utilizando el botón de retroalimentación.",
    spanish_latam: "No dude en proporcionar más detalles sobre el error utilizando el botón de feedback.",
    swedish: "Du är välkommen att ge mer detaljer om felet med hjälp av feedbackknappen.",
    thai: "อย่าลังเลที่จะให้รายละเอียดเพิ่มเติมเกี่ยวกับข้อผิดพลาดโดยใช้ปุ่มข้อเสนอแนะ",
    turkish: "Geri Bildirim Düğmesini kullanarak hatayla ilgili daha fazla detay sağlamaktan çekinmeyin.",
    ukrainian: "Не соромтеся надавати більше деталей про помилку, використовуючи кнопку відгуку.",
    vietnamese: "Cứ thoải mái cung cấp thêm chi tiết về lỗi bằng nút phản hồi.",
};

pub fn error_response(locale: Option<&str>) -> Result<InteractionResponse> {
    let error_embed = EmbedBuilder::new()
        .color(Color::Red.into())
        .thumbnail(ImageSource::url("https://cdn.lara.lv/emoji/grimacing.gif").unwrap())
        .title(EMBED_TITLE.get(locale))
        .description(EMBED_DESCRIPTION.get(locale))
        .footer(EmbedFooterBuilder::new(EMBED_FOOTER_TEXT.get(locale)))
        .build();

    Ok(InteractionResponseBuilder::send_message(
        InteractionResponseDataBuilder::new()
            .embeds([error_embed])
            .components(
                ComponentsBuilder::new()
                    .buttons(vec![
                        FeedbackButton::button((), locale)?,
                        support_server_invite(locale),
                    ])
                    .build(),
            )
            .build(),
    ))
}
