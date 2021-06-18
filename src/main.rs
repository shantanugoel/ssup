//! `ssup` is a simple cross-platform tool to send notifications via cli to local system or telegram.
//! It can also be used to get alerted after a long-running (or otherwise) command finishes. This can be used either by specifying the command at the end of the ssup command, or by chaining ssup with other commands. See the examples/CLI options below for more.
//! 
//! ## Examples
//! ```
//! # Send a notification to local system
//! $ ssup -m "Hello world!"
//! 
//! # Send a notification to telegram
//! $ export SSUP_TG_BOT_TOKEN=<Your telegram bot token>
//! $ ssup -m "Hello world!" -d telegram -c <your-telegram-chat-id> 
//! 
//! # Run a custom command and send notification after it finishes. Advantage of this is that ssup will also report whether command was successful or not.
//! $ ssup -m "Hello world!" sleep 5
//! 
//! # Alternate ways to run a custom command and send notification after it finishes
//! $ sleep 5; ssup -m "Hello world!"  # Always send notification
//! $ sleep 5 && ssup -m "Hello world!" # Send notification only on success
//! $ sleep 5 || ssup -m "Hello world!" # Send notification only on failure
//! 
//! ```
use std::env;
use std::error::Error;

use log::error;

mod cli;
mod telegram;

use telegram::Telegram;

static TOKEN_ENV_VAR: &str = "SSUP_TG_BOT_TOKEN";

fn notify_local(title: &str, message: &str) -> Result<(), notifica::Error> {
    notifica::notify(title, message)
}

fn notify_telegram(
    bot_token: Option<String>,
    chat_id: Option<String>,
    title: &str,
    message: &str,
) -> Result<(), Box<dyn Error>> {
    // TODO separate out init and do it in parallel with app run, where needed
    let token;
    match bot_token {
        Some(t) => token = t,
        _ => token = env::var(TOKEN_ENV_VAR)?,
    }
    Telegram::new(chat_id, token)?.send(title, message)
}

fn main() {
    env_logger::init();
    let opts: cli::Opts = cli::parse_opts();
    let mut title = opts.title.clone();

    if let Some(result) = opts.run_command() {
        match result {
            true => title += ": Successful",
            false => title += ": Unsuccessful",
        }
    }
    let msg_str = opts.message.as_str();
    let title_str = title.as_str();
    match opts.destination.as_str() {
        "local" => {
            if let Err(e) = notify_local(title_str, msg_str) {
                error!("{}", e);
            }
        }
        "telegram" => {
            if let Err(e) = notify_telegram(opts.bot_token, opts.chat_id, title_str, msg_str) {
                error!("{}", e);
            }
        }
        s => error!("Unrecognized target {}. This should never happen!", s),
    }
}
