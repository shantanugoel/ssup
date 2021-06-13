use std::process::Command;

use clap::{crate_authors, crate_description, crate_version, AppSettings, Clap};

/// Send a system notification
#[derive(Clap)]
#[clap(author = crate_authors!(), version=crate_version!(), about=crate_description!())]
#[clap(setting = AppSettings::ColoredHelp, setting = AppSettings::DeriveDisplayOrder)]

pub struct Opts {
    /// Optional title for the notification
    #[clap(short, long, default_value = "Sup!")]
    pub title: String,

    /// Notify with this message
    #[clap(short, long)]
    pub message: String,

    // Choose where to send the notification to
    #[clap(short, long, possible_values=&["local", "telegram"], default_value="local")]
    pub destination: String,

    /// {Telegram specific option} Chat id to which notification should be sent
    #[clap(long, required_if_eq("destination", "telegram"))]
    pub chat_id: Option<String>,

    /// {Telegram specific option } Optionally specify Telegram bot token in command instead of reading from env
    #[clap(long)]
    pub bot_token: Option<String>,

    /// Specify a command/executable to run and notify when it finishes
    #[clap()]
    pub command: Option<String>,
}

pub fn parse_opts() -> Opts {
    Opts::parse()
}

impl Opts {
    pub fn run_command(&self) -> Option<bool> {
        // TODO: Do this in a separate thread / async
        match &self.command {
            Some(run) => {
                let split_cmd: Vec<&str> = run.split(' ').collect();
                let executable = split_cmd[0];
                let mut cmd = Command::new(&executable);
                if split_cmd.len() > 1 {
                    cmd.args(&split_cmd[1..]);
                }

                Some(
                    cmd.status()
                        .expect("Could not parse command exit status")
                        .success(),
                )
            }
            _ => None,
        }
    }
}
