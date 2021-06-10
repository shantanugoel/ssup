pub struct Telegram {
    bot_token: String,
    chat_id: String,
}

impl Telegram {
    pub fn new() -> Telegram {
        Telegram {
            bot_token: String::from(""),
            chat_id: String::from(""),
        }
    }

    // TODO: Error handling
    pub fn init(&mut self, chat_id: String, bot_token: String) {
        self.chat_id = chat_id;
        self.bot_token = bot_token;
    }

    // https://api.telegram.org/bot<token>/sendMessage?chat_id=<group chat id >&text=<our text>
    pub fn send(&mut self, title: &str, msg: &str) -> Result<(), reqwest::Error> {
        let url: String = format!(
            "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text='{}: {}'",
            self.bot_token, self.chat_id, title, msg
        );
        // TODO error handling and make async
        reqwest::blocking::get(url)?.text()?;
        Ok(())
    }
}
