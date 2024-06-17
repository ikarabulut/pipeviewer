use std::fs::File;
use std::io;
use std::io::{BufWriter, Error, ErrorKind, Write};

pub fn write(outfile: &str, buffer: &[u8]) -> Result<bool, Error> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(io::stdout())
    };

    match writer.write_all(buffer) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == ErrorKind::BrokenPipe {
                return Ok(false);
            }
            return Err(e);
        }
    }
    Ok(true)
}
