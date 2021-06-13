use clap::{AppSettings, Clap};

/// Send a system notification
#[derive(Clap)]
#[clap(name = "sup", author = "Shantanu Goel <shantanugoel.com>")]
#[clap(setting = AppSettings::ColoredHelp, setting = AppSettings::DeriveDisplayOrder)]

pub struct Opts {
    /// Notify with this message
    #[clap(short, long)]
    pub message: String,

    // Choose where to send the notification to
    #[clap(short, long, possible_values=&["local", "telegram"], default_value="local")]
    pub target: String,

    /// Telegram chat id to which notification should be sent
    #[clap(long, required_if_eq("target", "telegram"))]
    pub chat_id: Option<String>,

    /// Optionally specify Telegram bot token in command instead of reading from env
    #[clap(long)]
    pub bot_token: Option<String>,

    /// Specify a command/executable to run and notify when it finishes
    #[clap()]
    pub command: Option<String>,
}

pub fn parse_opts() -> Opts {
    Opts::parse()
}
