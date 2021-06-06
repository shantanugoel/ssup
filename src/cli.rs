use clap::{AppSettings, Clap};

/// Send a system notification
#[derive(Clap)]
#[clap(name = "sup", author = "Shantanu Goel <shantanugoel.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// Notify with this message
    #[clap(short, long, default_value = "Hey!")]
    pub message: String,

    /// Specify an executable to run and notify when it finishes
    #[clap(short, long)]
    pub run: Option<String>,
}

pub fn parse_opts() -> Opts {
    Opts::parse()
}
