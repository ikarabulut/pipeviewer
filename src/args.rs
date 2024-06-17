use clap::{Arg, Command};
use std::env;

pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub silent: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches = Command::new("pipeviewer")
            .arg(Arg::new("infile").help("Read a file instead of stdin"))
            .arg(
                Arg::new("outfile")
                    .short('o')
                    .long("outfile")
                    .value_name("Outfile")
                    .help("Write output to a file instead of stdout")
                    .action(clap::ArgAction::Set), // Specify that this argument takes a value
            )
            .arg(
                Arg::new("silent")
                    .short('s')
                    .long("silent")
                    .action(clap::ArgAction::SetTrue),
            ) // Specify that this argument is a flag
            .get_matches();

        let infile = matches
            .get_one::<String>("infile")
            .map(|s| s.as_str())
            .unwrap_or_default()
            .to_string();
        let outfile = matches
            .get_one::<String>("outfile")
            .map(|s| s.as_str())
            .unwrap_or_default()
            .to_string();
        let silent = if matches.get_flag("silent") {
            true
        } else {
            !env::var("PV_SILENT").unwrap_or_default().is_empty()
        };
        Self {
            infile,
            outfile,
            silent,
        }
    }
}
