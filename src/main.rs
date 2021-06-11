use std::env;
use std::error::Error;
use std::process::Command;

mod cli;
mod telegram;

use telegram::Telegram;

static TOKEN_ENV_VAR: &str = "SUP_TG_BOT_TOKEN";

// TODO add error handling/returning
fn notify_local(title: &str, message: &str) -> Result<(), notifica::Error> {
    notifica::notify(title, message)
}

fn notify_telegram(
    tg_opts: cli::Telegram,
    title: &str,
    message: &str,
) -> Result<(), env::VarError> {
    // TODO separate out init and do it in parallel with app run, where needed
    let bot_token;
    match tg_opts.tg_bot_token_env {
        true => bot_token = env::var(TOKEN_ENV_VAR)?,
        false => bot_token = tg_opts.tg_bot_token.unwrap(), // TODO: Error handling
    }
    let mut tg = Telegram::new();
    tg.init(tg_opts.tg_chat_id, bot_token);

    // TODO Use/fix error handling
    tg.send(title, message);

    Ok(())
}

fn main() {
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

        let status = cmd.status().unwrap();

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
                println!("sup error: {}", e);
            }
        }
        cli::SubCommand::Telegram(tg_opts) => {
            if let Err(e) = notify_telegram(tg_opts, title.as_str(), opts.message.as_str()) {
                println!("sup error: {}", e);
            }
        }
    }
}
