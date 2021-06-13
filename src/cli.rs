use clap::{AppSettings, Clap};

/// Send a system notification
#[derive(Clap)]
#[clap(name = "sup", author = "Shantanu Goel <shantanugoel.com>")]
#[clap(setting = AppSettings::ColoredHelp,
     setting=AppSettings::DeriveDisplayOrder)]
pub struct Opts {
    /// Notify with this message
    #[clap(short, long, default_value = "Hey!")]
    pub message: String,

    /// Specify an executable to run and notify when it finishes
    #[clap(short, long)]
    pub run: Option<String>,

    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

/// Notifications should be sent to local system
#[derive(Clap)]
pub enum SubCommand {
    Local(Local),
    Telegram(Telegram),
}

#[derive(Clap)]
pub struct Local {}

#[derive(Clap)]
/// Send notification to telegram. Reads the bot token value from
/// SUP_TG_BOT_TOKEN env variable unless it is provided on the
/// command line with tg-bot-token option
pub struct Telegram {
    /// Telegram chat id to which notification should be sent.
    #[clap(long)]
    pub chat_id: String,

    /// Specify Telegram bot token in command
    #[clap(long)]
    pub bot_token: Option<String>,
}

pub fn parse_opts() -> Opts {
    Opts::parse()
}
