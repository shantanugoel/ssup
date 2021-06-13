use thiserror::Error;

pub struct Telegram {
    bot_token: String,
    chat_id: String,
}

#[derive(Error, Debug)]
pub enum TelegramError {
    #[error("Invalid Chat Id")]
    InvalidChatId(),
    #[error("Error from Telgram API call: {0}")]
    ApiGetError(String),
}

impl Telegram {
    pub fn new(
        chat_id: Option<String>,
        bot_token: String,
    ) -> Result<Telegram, Box<dyn std::error::Error>> {
        match chat_id {
            Some(c) => Ok(Telegram {
                bot_token,
                chat_id: c,
            }),
            _ => Err(Box::new(TelegramError::InvalidChatId())),
        }
    }

    pub fn send(&self, title: &str, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
        // https://api.telegram.org/bot<token>/sendMessage?chat_id=<group chat id >&text=<our text>
        let url: String = format!(
            "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text='{}: {}'",
            self.bot_token, self.chat_id, title, msg
        );

        // TODO make async
        let rsp = reqwest::blocking::get(url)?;
        match rsp.status().is_success() {
            false => Err(Box::new(TelegramError::ApiGetError(rsp.text()?))),
            true => Ok(()),
        }
    }
}
