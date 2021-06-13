use std::env;
use std::error::Error;
use std::process::Command;

use log::error;

mod cli;
mod telegram;

use telegram::Telegram;

static TOKEN_ENV_VAR: &str = "SUP_TG_BOT_TOKEN";

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
    let mut title = String::from("Sup!");

    // TODO: Do this in a separate thread / async
    if let Some(run) = opts.command {
        let split_cmd: Vec<&str> = run.split(' ').collect();
        let executable = split_cmd[0];
        let mut cmd = Command::new(&executable);
        if split_cmd.len() > 1 {
            cmd.args(&split_cmd[1..]);
        }

        let status = cmd.status().expect("Could not parse command exit status");

        title.clear();
        title.push_str(executable);
        if status.success() {
            title += ": Successful";
        } else {
            title += ": Unsuccessful"
        }
    }

    let msg_str = opts.message.as_str();
    let title_str = title.as_str();
    match opts.target.as_str() {
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
