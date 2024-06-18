use std::fs::File;
use std::io;
use std::io::{BufWriter, Error, ErrorKind, Write};
use std::sync::{Arc, Mutex};

pub fn write_loop(outfile: &str, quit: Arc<Mutex<bool>>) -> Result<(), Error> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(io::stdout())
    };

    loop {
        let buffer: Vec<u8> = Vec::new();
        {
            let quit = quit.lock().unwrap();
            if *quit {
                break;
            }
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
