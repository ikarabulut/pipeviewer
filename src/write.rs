use crossbeam::channel::Receiver;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Error, ErrorKind, Write};

pub fn write_loop(outfile: &str, write_rx: Receiver<Vec<u8>>) -> Result<(), Error> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(io::stdout())
    };

    loop {
        let buffer = write_rx.recv().unwrap();
        if buffer.is_empty() {
            break;
        }
        match writer.write_all(&buffer) {
            Ok(_) => {}
            Err(e) => {
                if e.kind() == ErrorKind::BrokenPipe {
                    return Ok(());
                }
                return Err(e);
            }
        }
    }
    Ok(())
}
