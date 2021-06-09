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
    subcmd: SubCommand,
}

/// Notifications should be sent to local system
#[derive(Clap)]
enum SubCommand {
    Local(Local),
    Telegram(Telegram),
}

#[derive(Clap)]
struct Local {}

#[derive(Clap)]
struct Telegram {
    /// Telegram chat id to which notification should be sent.
    #[clap(long)]
    pub tg_chat_id: String,

    /// Specify Telegram bot token in command
    #[clap(
        long,
        conflicts_with("tg-bot-token-env"),
        required_unless_present("tg-bot-token-env")
    )]
    pub tg_bot_token: Option<String>,

    /// Take Telegram bot token from SUP_TG_BOT_TOKEN env variable
    #[clap(long, required_unless_present("tg-bot-token"))]
    pub tg_bot_token_env: bool,
}

pub fn parse_opts() -> Opts {
    Opts::parse()
}
