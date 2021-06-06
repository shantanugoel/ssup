mod cli;

fn main() -> Result<(), notifica::Error> {
    let opts: cli::Opts = cli::parse_opts();
    notifica::notify("Sup!", opts.message.as_str())
}
