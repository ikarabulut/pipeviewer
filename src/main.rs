use clap::{Arg, Command};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Write};

const CHUNK_SIZE: usize = 6 * 1024;

fn main() -> io::Result<()> {
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
        .unwrap_or_default();
    let outfile = matches
        .get_one::<String>("outfile")
        .map(|s| s.as_str())
        .unwrap_or_default();
    let silent = if matches.get_flag("silent") {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };

    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(io::stdin())
    };

    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(io::stdout())
    };

    let mut buffer = [0; CHUNK_SIZE];
    let mut total_bytes = 0;
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        total_bytes += num_read;
        if !silent {
            eprint!("\r{}", total_bytes)
        };

        match writer.write_all(&buffer[..num_read]) {
            Ok(_) => {}
            Err(e) => {
                if e.kind() == ErrorKind::BrokenPipe {
                    break;
                }
                return Err(e);
            }
        }
    }

    Ok(())
}
