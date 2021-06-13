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
    tg_opts: cli::Telegram,
    title: &str,
    message: &str,
) -> Result<(), Box<dyn Error>> {
    // TODO separate out init and do it in parallel with app run, where needed
    let bot_token;
    match tg_opts.bot_token {
        Some(token) => bot_token = token,
        _ => bot_token = env::var(TOKEN_ENV_VAR)?,
    }
    let tg = Telegram::new(tg_opts.chat_id, bot_token);

    tg.send(title, message)
}

fn main() {
    env_logger::init();
    let opts: cli::Opts = cli::parse_opts();
    let mut title = String::from("Sup!");

    // TODO: Do this in a separate thread / async
    if let Some(run) = opts.run {
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

    match opts.subcmd {
        cli::SubCommand::Local(_) => {
            if let Err(e) = notify_local(title.as_str(), opts.message.as_str()) {
                error!("{}", e);
            }
        }
        cli::SubCommand::Telegram(tg_opts) => {
            if let Err(e) = notify_telegram(tg_opts, title.as_str(), opts.message.as_str()) {
                error!("sup error: {}", e);
            }
        }
    }
}
