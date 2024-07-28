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
    localization::LocalizedText,
    option_ext::OptionExt,
    sentry_user_feedback::FeedbackClient,
};

// noinspection SpellCheckingInspection
const TITLE: LocalizedText = LocalizedText {
    bulgarian: ":bellhop_bell: Формуляр за обратна връзка",
    chinese_cn: ":bellhop_bell: 反馈表",
    chinese_tw: ":bellhop_bell: 反饋表",
    croatian: ":bellhop_bell: Obrazac za povratne informacije",
    czech: ":bellhop_bell: Formulář zpětné vazby",
    danish: ":bellhop_bell: Feedbackformular",
    dutch: ":bellhop_bell: Feedbackformulier",
    english_uk: ":bellhop_bell: Feedback Form",
    english_us: ":bellhop_bell: Feedback Form",
    finnish: ":bellhop_bell: Palautekaavake",
    french: ":bellhop_bell: Formulaire de retour",
    german: ":bellhop_bell: Feedback-Formular",
    greek: ":bellhop_bell: Έντυπο Σχολίων",
    hindi: ":bellhop_bell: प्रतिक्रिया फ़ॉर्म",
    hungarian: ":bellhop_bell: Visszajelzési űrlap",
    indonesian: ":bellhop_bell: Formulir Umpan Balik",
    italian: ":bellhop_bell: Modulo di feedback",
    japanese: ":bellhop_bell: フィードバックフォーム",
    korean: ":bellhop_bell: 피드백 양식",
    lithuanian: ":bellhop_bell: Atsiliepimų forma",
    norwegian: ":bellhop_bell: Tilbakemeldingsskjema",
    polish: ":bellhop_bell: Formularz opinii",
    portuguese_br: ":bellhop_bell: Formulário de Feedback",
    romanian: ":bellhop_bell: Formular de feedback",
    russian: ":bellhop_bell: Форма обратной связи",
    spanish: ":bellhop_bell: Formulario de retroalimentación",
    spanish_latam: ":bellhop_bell: Formulario de feedback",
    swedish: ":bellhop_bell: Feedbackformulär",
    thai: ":bellhop_bell: แบบฟอร์มข้อเสนอแนะ",
    turkish: ":bellhop_bell: Geribildirim Formu",
    ukrainian: ":bellhop_bell: Форма відгуків",
    vietnamese: ":bellhop_bell: Mẫu Phản Hồi",
};

// noinspection SpellCheckingInspection
const CONTENT_INPUT_LABEL: LocalizedText = LocalizedText {
    bulgarian: "Обратна връзка",
    chinese_cn: "反馈",
    chinese_tw: "反饋",
    croatian: "Povratne informacije",
    czech: "Zpětná vazba",
    danish: "Feedback",
    dutch: "Feedback",
    english_uk: "Feedback",
    english_us: "Feedback",
    finnish: "Palaute",
    french: "Retour",
    german: "Feedback",
    greek: "Ανατροφοδότηση",
    hindi: "प्रतिक्रिया",
    hungarian: "Visszajelzés",
    indonesian: "Umpan balik",
    italian: "Feedback",
    japanese: "フィードバック",
    korean: "피드백",
    lithuanian: "Atsiliepimai",
    norwegian: "Tilbakemelding",
    polish: "Opinie",
    portuguese_br: "Feedback",
    romanian: "Feedback",
    russian: "Отзыв",
    spanish: "Retroalimentación",
    spanish_latam: "Feedback",
    swedish: "Feedback",
    thai: "ข้อเสนอแนะ",
    turkish: "Geri bildirim",
    ukrainian: "Відгук",
    vietnamese: "Phản hồi",
};

// noinspection SpellCheckingInspection
const CONTACT_INPUT_LABEL: LocalizedText = LocalizedText {
    bulgarian: "Контактна информация",
    chinese_cn: "联系信息",
    chinese_tw: "聯絡資訊",
    croatian: "Kontaktne informacije",
    czech: "Kontaktní informace",
    danish: "Kontaktinformation",
    dutch: "Contactinformatie",
    english_uk: "Contact information",
    english_us: "Contact information",
    finnish: "Yhteystiedot",
    french: "Informations de contact",
    german: "Kontaktinformationen",
    greek: "Στοιχεία επικοινωνίας",
    hindi: "संपर्क जानकारी",
    hungarian: "Elérhetőségi adatok",
    indonesian: "Informasi kontak",
    italian: "Informazioni di contatto",
    japanese: "連絡先情報",
    korean: "연락처 정보",
    lithuanian: "Kontaktinė informacija",
    norwegian: "Kontaktinformasjon",
    polish: "Informacje kontaktowe",
    portuguese_br: "Informações de contato",
    romanian: "Informații de contact",
    russian: "Контактная информация",
    spanish: "Información de contacto",
    spanish_latam: "Información de contacto",
    swedish: "Kontaktinformation",
    thai: "ข้อมูลติดต่อ",
    turkish: "İletişim bilgileri",
    ukrainian: "Контактна інформація",
    vietnamese: "Thông tin liên hệ",
};

// noinspection SpellCheckingInspection
const CONTACT_INPUT_PLACEHOLDER: LocalizedText = LocalizedText {
    bulgarian: "Потребителско име в Discord, имейл и др. (По избор)",
    chinese_cn: "Discord 用户名、邮箱等（可选）",
    chinese_tw: "Discord 使用者名稱、電郵等（選填）",
    croatian: "Discord korisničko ime, e-mail itd. (Neobavezno)",
    czech: "Uživatelské jméno na Discordu, e-mail atd. (Volitelné)",
    danish: "Discord brugernavn, e-mail osv. (Valgfrit)",
    dutch: "Discord-gebruikersnaam, e-mail enz. (Optioneel)",
    english_uk: "Discord username, email etc. (Optional)",
    english_us: "Discord username, email etc. (Optional)",
    finnish: "Discord-käyttäjänimi, sähköposti jne. (Vapaaehtoinen)",
    french: "Nom d'utilisateur Discord, e-mail, etc. (Facultatif)",
    german: "Discord-Benutzername, E-Mail usw. (Optional)",
    greek: "Όνομα χρήστη Discord, email κλπ. (Προαιρετικό)",
    hindi: "डिस्कॉर्ड उपयोगकर्ता नाम, ईमेल आदि (वैकल्पिक)",
    hungarian: "Discord felhasználónév, e-mail stb. (Opcionális)",
    indonesian: "Nama pengguna Discord, email, dll. (Opsional)",
    italian: "Nome utente Discord, email, ecc. (Opzionale)",
    japanese: "Discordのユーザー名、メールアドレスなど（任意）",
    korean: "디스코드 사용자 이름, 이메일 등 (선택 사항)",
    lithuanian: "Discord vartotojo vardas, el. paštas ir kt. (Nebūtina)",
    norwegian: "Discord brukernavn, e-post osv. (Valgfritt)",
    polish: "Nazwa użytkownika na Discord, e-mail itp. (Opcjonalnie)",
    portuguese_br: "Nome de usuário do Discord, e-mail etc. (Opcional)",
    romanian: "Nume de utilizator Discord, email etc. (Opțional)",
    russian: "Имя пользователя в Discord, электронная почта и т.д. (Необязательно)",
    spanish: "Nombre de usuario de Discord, correo electrónico, etc. (Opcional)",
    spanish_latam: "Nombre de usuario de Discord, correo electrónico, etc. (Opcional)",
    swedish: "Discord-användarnamn, e-post etc. (Frivilligt)",
    thai: "ชื่อผู้ใช้ Discord, อีเมล ฯลฯ (ไม่จำเป็น)",
    turkish: "Discord kullanıcı adı, e-posta vb. (İsteğe bağlı)",
    ukrainian: "Ім'я користувача в Discord, електронна пошта тощо. (Необов'язково)",
    vietnamese: "Tên người dùng Discord, email, v.v. (Tùy chọn)",
};

// noinspection SpellCheckingInspection
const EMBED_TITLE: LocalizedText = LocalizedText {
    bulgarian: "Обратната връзка е изпратена!",
    chinese_cn: "反馈已提交！",
    chinese_tw: "已提交反饋！",
    croatian: "Povratne informacije poslane!",
    czech: "Zpětná vazba odeslána!",
    danish: "Feedback indsendt!",
    dutch: "Feedback ingediend!",
    english_uk: "Feedback submitted!",
    english_us: "Feedback submitted!",
    finnish: "Palaute lähetetty!",
    french: "Retour envoyé!",
    german: "Feedback eingereicht!",
    greek: "Το σχόλιο υποβλήθηκε!",
    hindi: "प्रतिक्रिया सबमिट की गई!",
    hungarian: "Visszajelzés elküldve!",
    indonesian: "Umpan balik terkirim!",
    italian: "Feedback inviato!",
    japanese: "フィードバックが送信されました！",
    korean: "피드백 제출됨!",
    lithuanian: "Atsiliepimas pateiktas!",
    norwegian: "Tilbakemelding sendt!",
    polish: "Opinia została wysłana!",
    portuguese_br: "Feedback enviado!",
    romanian: "Feedback trimis!",
    russian: "Отзыв отправлен!",
    spanish: "¡Feedback enviado!",
    spanish_latam: "¡Feedback enviado!",
    swedish: "Feedback inskickad!",
    thai: "ส่งข้อเสนอแนะแล้ว!",
    turkish: "Geri bildirim gönderildi!",
    ukrainian: "Відгук надіслано!",
    vietnamese: "Đã gửi phản hồi!",
};

// noinspection SpellCheckingInspection
#[rustfmt::skip]
const EMBED_DESCRIPTION: LocalizedText = LocalizedText {
    bulgarian: "Благодарим ви за обратната връзка! Вашата обратна връзка оформя бъдещето на бота.\n\n> *Ако сте предоставили потребителското си име в Discord като контактна информация, моля приемете покани за приятелство от **laralove***.",
    chinese_cn: "感谢您的反馈！您的反馈将塑造机器人的未来。\n\n> *如果您提供了您的Discord用户名作为联系信息，请接受来自**laralove**的好友请求。*",
    chinese_tw: "感謝您的反饋！您的反饋將塑造機器人的未來。\n\n> *如果您提供了您的Discord使用者名稱作為聯絡資訊，請接受來自**laralove**的好友請求。*",
    croatian: "Hvala vam na povratnim informacijama! Vaše povratne informacije oblikuju budućnost bota.\n\n> *Ako ste naveli svoje korisničko ime na Discordu kao kontakt informaciju, molimo prihvatite zahtjeve za prijateljstvo od **laralove***.",
    czech: "Děkujeme za vaši zpětnou vazbu! Vaše zpětná vazba formuje budoucnost bota.\n\n> *Pokud jste poskytli své uživatelské jméno na Discord jako kontaktní informaci, přijměte prosím žádosti o přátelství od **laralove***.",
    danish: "Tak for din feedback! Din feedback former fremtiden for botten.\n\n> *Hvis du har opgivet dit Discord brugernavn som din kontaktinformation, accepter venligst venneanmodninger fra **laralove***.",
    dutch: "Bedankt voor je feedback! Jouw feedback vormt de toekomst van de bot.\n\n> *Als je je Discord-gebruikersnaam hebt opgegeven als contactinformatie, accepteer dan vriendschapsverzoeken van **laralove***.",
    english_uk: "Thank you for your feedback! Your feedback shapes the future of the bot.\n\n> *If you provided your Discord username as your contact information, please accept friend requests from **laralove***.",
    english_us: "Thank you for your feedback! Your feedback shapes the future of the bot.\n\n> *If you provided your Discord username as your contact information, please accept friend requests from **laralove***.",
    finnish: "Kiitos palautteestasi! Palautteesi muokkaa botin tulevaisuutta.\n\n> *Jos annoit Discord-käyttäjänimesi yhteystietona, hyväksy kaveripyyntöjä käyttäjältä **laralove***.",
    french: "Merci pour votre retour! Votre feedback façonne l'avenir du bot.\n\n> *Si vous avez fourni votre nom d'utilisateur Discord comme information de contact, veuillez accepter les demandes d'amitié de **laralove***.",
    german: "Vielen Dank für Ihr Feedback! Ihr Feedback gestaltet die Zukunft des Bots.\n\n> *Wenn Sie Ihren Discord-Benutzernamen als Kontaktinformation angegeben haben, bitte akzeptieren Sie Freundschaftsanfragen von **laralove***.",
    greek: "Ευχαριστούμε για τα σχόλιά σας! Τα σχόλιά σας διαμορφώνουν το μέλλον του bot.\n\n> *Αν δώσατε το όνομα χρήστη σας στο Discord ως πληροφορία επικοινωνίας, παρακαλούμε δεχτείτε αιτήσεις φιλίας από τον **laralove***.",
    hindi: "आपकी प्रतिक्रिया के लिए धन्यवाद! आपकी प्रतिक्रिया बॉट के भविष्य को आकार देती है।\n\n> *यदि आपने अपना Discord उपयोगकर्ता नाम संपर्क जानकारी के रूप में प्रदान किया है, तो कृपया **laralove** से मित्र अनुरोध स्वीकार करें।*",
    hungarian: "Köszönjük visszajelzését! A visszajelzése formálja a bot jövőjét.\n\n> *Ha megadta Discord felhasználónevét kapcsolattartási adatként, kérem fogadja el a barátkérelmeket **laralove**-tól.*",
    indonesian: "Terima kasih atas tanggapan Anda! Tanggapan Anda membentuk masa depan bot.\n\n> *Jika Anda menyediakan nama pengguna Discord Anda sebagai informasi kontak, silakan terima permintaan pertemanan dari **laralove***.",
    italian: "Grazie per il tuo feedback! Il tuo feedback modella il futuro del bot.\n\n> *Se hai fornito il tuo username Discord come informazione di contatto, accetta le richieste di amicizia da **laralove***.",
    japanese: "フィードバックありがとうございます！あなたのフィードバックがボットの未来を形作ります。\n\n> *連絡先情報としてDiscordのユーザー名を提供した場合、**laralove**からのフレンドリクエストを承認してください。*",
    korean: "피드백을 주셔서 감사합니다! 귀하의 피드백이 봇의 미래를 형성합니다.\n\n> *연락처 정보로 Discord 사용자 이름을 제공한 경우, **laralove**의 친구 요청을 수락해 주세요.*",
    lithuanian: "Dėkojame už jūsų atsiliepimus! Jūsų atsiliepimai formuoja būsimą botą.\n\n> *Jei nurodėte savo „Discord“ vartotojo vardą kaip kontaktinę informaciją, prašome priimti draugų užklausas iš **laralove***.",
    norwegian: "Takk for tilbakemeldingen din! Din tilbakemelding former fremtiden for botten.\n\n> *Hvis du oppga Discord-brukernavnet ditt som kontaktinformasjon, vennligst aksepter vennforespørsler fra **laralove***.",
    polish: "Dziękujemy za Twoją opinię! Twoja opinia kształtuje przyszłość bota.\n\n> *Jeśli podałeś swoją nazwę użytkownika na Discord jako informację kontaktową, proszę zaakceptuj zaproszenia do znajomych od **laralove***.",
    portuguese_br: "Obrigado pelo seu feedback! Seu feedback molda o futuro do bot.\n\n> *Se você forneceu seu nome de usuário do Discord como sua informação de contato, por favor aceite solicitações de amizade de **laralove***.",
    romanian: "Vă mulțumim pentru feedback! Feedback-ul dumneavoastră modelează viitorul botului.\n\n> *Dacă ați furnizat numele dvs. de utilizator Discord ca informație de contact, vă rugăm să acceptați cererile de prietenie de la **laralove***.",
    russian: "Спасибо за ваш отзыв! Ваш отзыв формирует будущее бота.\n\n> *Если вы указали свое имя пользователя в Discord в качестве контактной информации, пожалуйста, принимайте запросы в друзья от **laralove***.",
    spanish: "¡Gracias por tu feedback! Tu feedback moldea el futuro del bot.\n\n> *Si proporcionaste tu nombre de usuario de Discord como tu información de contacto, por favor acepta solicitudes de amistad de **laralove***.",
    spanish_latam: "¡Gracias por tu retroalimentación! Tu retroalimentación moldea el futuro del bot.\n\n> *Si proporcionaste tu nombre de usuario de Discord como tu información de contacto, por favor acepta solicitudes de amistad de **laralove***.",
    swedish: "Tack för din feedback! Din feedback formar framtiden för botten.\n\n> *Om du angav ditt Discord-användarnamn som din kontaktinformation, vänligen acceptera vänförfrågningar från **laralove***.",
    thai: "ขอบคุณสำหรับข้อเสนอแนะของคุณ! ข้อเสนอแนะของคุณช่วยกำหนดอนาคตของบอท\n\n> *หากคุณได้ระบุชื่อผู้ใช้ Discord ของคุณเป็นข้อมูลการติดต่อ โปรดยอมรับคำขอเป็นเพื่อนจาก **laralove***.",
    turkish: "Geri bildiriminiz için teşekkür ederiz! Geri bildiriminiz botun geleceğini şekillendiriyor.\n\n> *İletişim bilgileri olarak Discord kullanıcı adınızı verdiyseniz, lütfen **laralove**'tan arkadaşlık isteklerini kabul edin.*",
    ukrainian: "Дякуємо за ваш відгук! Ваш відгук формує майбутнє бота.\n\n> *Якщо ви надали своє ім'я користувача в Discord як контактну інформацію, будь ласка, прийміть запити на дружбу від **laralove***.",
    vietnamese: "Cảm ơn bạn đã gửi phản hồi! Phản hồi của bạn hình thành tương lai của bot.\n\n> *Nếu bạn đã cung cấp tên người dùng Discord của mình làm thông tin liên lạc, vui lòng chấp nhận lời mời kết bạn từ **laralove***.",
};

// noinspection SpellCheckingInspection
#[rustfmt::skip]
const EMBED_FOOTER: LocalizedText = LocalizedText {
    bulgarian: "Не се колебайте да се присъедините към сървъра за поддръжка, за да взаимодействате с общността на бота.",
    chinese_cn: "随时加入支持服务器，与机器人社区互动。",
    chinese_tw: "隨時加入支援伺服器，與機器人社群互動。",
    croatian: "Slobodno se pridružite podrškom serveru kako biste komunicirali s zajednicom bota.",
    czech: "Klidně se připojte na podpůrný server, abyste mohli interagovat s komunitou bota.",
    danish: "Du er velkommen til at deltage i supportserveren for at interagere med botens fællesskab.",
    dutch: "Voel je vrij om je aan te sluiten bij de supportserver om te interageren met de gemeenschap van de bot.",
    english_uk: "Feel free to join the support server to interact with the bot's community.",
    english_us: "Feel free to join the support server to interact with the bot's community.",
    finnish: "Liity tukipalvelimelle vapaasti keskustellaksesi botin yhteisön kanssa.",
    french: "N'hésitez pas à rejoindre le serveur de support pour interagir avec la communauté du bot.",
    german: "Fühlen Sie sich frei, dem Support-Server beizutreten, um mit der Bot-Gemeinschaft zu interagieren.",
    greek: "Μη διστάσετε να συνδεθείτε στον διακομιστή υποστήριξης για να αλληλεπιδράσετε με την κοινότητα του bot.",
    hindi: "बॉट के समुदाय के साथ बातचीत करने के लिए सहायता सर्वर में शामिल होने में संकोच न करें।",
    hungarian: "Nyugodtan csatlakozz a támogatói szerverhez, hogy interakcióba léphess a bot közösségével.",
    indonesian: "Jangan ragu untuk bergabung dengan server dukungan untuk berinteraksi dengan komunitas bot.",
    italian: "Sentiti libero di unirti al server di supporto per interagire con la comunità del bot.",
    japanese: "サポートサーバーに参加して、ボットのコミュニティと交流してください。",
    korean: "지원 서버에 자유롭게 참여하여 봇 커뮤니티와 상호 작용하세요.",
    lithuanian: "Prisijunkite prie palaikymo serverio, kad galėtumėte bendrauti su botos bendruomene.",
    norwegian: "Føl deg fri til å bli med på støtteserveren for å samhandle med botens samfunn.",
    polish: "Dołącz do serwera wsparcia, aby wchodzić w interakcję ze społecznością bota.",
    portuguese_br: "Sinta-se à vontade para entrar no servidor de suporte para interagir com a comunidade do bot.",
    romanian: "Nu ezitați să vă alăturați serverului de suport pentru a interacționa cu comunitatea botului.",
    russian: "Не стесняйтесь присоединяться к серверу поддержки, чтобы взаимодействовать с сообществом бота.",
    spanish: "Únete libremente al servidor de soporte para interactuar con la comunidad del bot.",
    spanish_latam: "Únete libremente al servidor de soporte para interactuar con la comunidad del bot.",
    swedish: "Känn dig fri att gå med i supportservern för att interagera med botens gemenskap.",
    thai: "อย่าลังเลที่จะเข้าร่วมเซิร์ฟเวอร์สนับสนุนเพื่อโต้ตอบกับชุมชนของบอท",
    turkish: "Bot topluluğu ile etkileşimde bulunmak için destek sunucusuna katılmaktan çekinmeyin.",
    ukrainian: "Не соромтеся приєднуватися до сервера підтримки, щоб взаємодіяти з спільнотою бота.",
    vietnamese: "Cứ thoải mái tham gia máy chủ hỗ trợ để tương tác với cộng đồng bot.",
};

struct ContactTextInput;

impl CreateTextInput for ContactTextInput {
    const CUSTOM_ID: &'static str = "contact";

    fn text_input(locale: Option<&str>) -> Result<TextInput> {
        Ok(TextInputBuilder::new(
            CONTACT_INPUT_LABEL.get(locale).to_owned(),
            Self::CUSTOM_ID.to_owned(),
        )
        .placeholder(CONTACT_INPUT_PLACEHOLDER.get(locale).to_owned())
        .build())
    }
}

struct ContentTextInput;

impl CreateTextInput for ContentTextInput {
    const CUSTOM_ID: &'static str = "content";

    fn text_input(locale: Option<&str>) -> Result<TextInput> {
        Ok(TextInputBuilder::new(
            CONTENT_INPUT_LABEL.get(locale).to_owned(),
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
    locale: Option<String>,
}

impl CreateModal for FeedbackModal {
    fn show_response(locale: Option<&str>) -> Result<InteractionResponse> {
        Ok(InteractionResponseBuilder::show_modal(
            TITLE.get(locale).to_owned(),
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
            locale: interaction.locale,
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
            .title(EMBED_TITLE.get(self.locale.as_deref()))
            .description(EMBED_DESCRIPTION.get(self.locale.as_deref()))
            .footer(EmbedFooterBuilder::new(
                EMBED_FOOTER.get(self.locale.as_deref()),
            ))
            .build();

        handle
            .respond(InteractionResponseBuilder::send_message(
                InteractionResponseDataBuilder::new()
                    .embeds([embed])
                    .components(
                        ComponentsBuilder::new()
                            .buttons(vec![support_server_invite(self.locale.as_deref())])
                            .build(),
                    )
                    .build(),
            ))
            .await?;

        Ok(())
    }
}
