use std::env;
use std::io::{self, Read, Write};
use clap::{Command, Arg};

const CHUNK_SIZE: usize = 6 * 1024;

fn main() {
    let mut total_bytes = 0;
    let matches = Command::new("pipeviewer")
        .arg(Arg::new("infile").help("Read a file instead of stdin"))
        .arg(
            Arg::new("outfile")
            .short('o')
            .long("outfile")
            .value_name("Outfile")
            .help("Write output to a file instead of stdout")
            .action(clap::ArgAction::Set)  // Specify that this argument takes a value
        )
        .arg(Arg::new("silent")
            .short('s')
            .long("silent")
            .action(clap::ArgAction::SetTrue))  // Specify that this argument is a flag
        .get_matches();

    let infile = matches.get_one::<String>("infile").map(|s| s.as_str()).unwrap_or_default();
    let outfile = matches.get_one::<String>("outfile").map(|s| s.as_str()).unwrap_or_default();
    let silent = if matches.get_flag("silent") {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };
    loop {
        let mut buffer = [0; CHUNK_SIZE];

        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        total_bytes += num_read;
        io::stdout().write_all(&buffer[..num_read]).unwrap();
    }
    eprintln!("{}", total_bytes);
}
