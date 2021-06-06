use std::process::Command;

mod cli;

fn main() -> Result<(), notifica::Error> {
    let opts: cli::Opts = cli::parse_opts();
    let mut title = String::from("Sup!");
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
    notifica::notify(title.as_str(), opts.message.as_str())
}
