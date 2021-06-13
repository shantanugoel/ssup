use std::error::Error;
use std::fmt;

pub struct Telegram {
    bot_token: String,
    chat_id: String,
}

#[derive(Debug)]
struct TgSendError {
    msg: String,
}

impl fmt::Display for TgSendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = format!("Error returned by telegram API call: {}", self.msg);
        write!(f, "{}", error)
    }
}

impl std::error::Error for TgSendError {}

impl Telegram {
    pub fn new(chat_id: String, bot_token: String) -> Telegram {
        Telegram {
            bot_token: bot_token,
            chat_id: chat_id,
        }
    }

    pub fn send(&self, title: &str, msg: &str) -> Result<(), Box<dyn Error>> {
        // https://api.telegram.org/bot<token>/sendMessage?chat_id=<group chat id >&text=<our text>
        let url: String = format!(
            "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text='{}: {}'",
            self.bot_token, self.chat_id, title, msg
        );

        // TODO make async
        let rsp = reqwest::blocking::get(url)?;
        match rsp.status().is_success() {
            false => Err(Box::new(TgSendError { msg: rsp.text()? })),
            true => Ok(()),
        }
    }
}
