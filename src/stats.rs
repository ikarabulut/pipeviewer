mod timer;

use crossbeam::channel::Receiver;
use crossterm::style::Stylize;
use crossterm::{
    cursor, execute,
    style::{self, Color, PrintStyledContent},
    terminal::{Clear, ClearType},
};
use std::io;
use std::io::{Result, Stderr, Write};
use std::time::Instant;
use timer::Timer;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    let start = Instant::now();
    let mut timer = Timer::new();
    let mut stderr = io::stderr();
    loop {
        let num_bytes = stats_rx.recv().unwrap();
        timer.update();
        let rates_per_second = num_bytes as f64 / timer.delta.as_secs_f64();
        total_bytes += num_bytes;
        if !silent && timer.ready {
            timer.ready = false;
            output_progress(
                &mut stderr,
                total_bytes,
                start.elapsed().as_secs().as_time(),
                rates_per_second,
            );
        };
        if num_bytes == 0 {
            break;
        }
    }
    if !silent {
        eprintln!();
    }
    Ok(())
}

fn output_progress(stderr: &mut Stderr, bytes: usize, elapsed: String, rate: f64) {
    let bytes = style::style(format!("{} ", bytes)).with(Color::Red);
    let elapsed = style::style(elapsed).with(Color::Green);
    let rate = style::style(format!(" [{:.0}b/s]", rate).with(Color::Blue));
    let _ = execute!(
        stderr,
        cursor::MoveToColumn(0),
        Clear(ClearType::CurrentLine),
        PrintStyledContent(bytes),
        PrintStyledContent(elapsed),
        PrintStyledContent(rate)
    );
    let _ = stderr.flush();
}

trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (min, sec) = (left / 60, left % 60);
        format!("{}:{:02}:{:02}", hours, min, sec)
    }
}

#[cfg(test)]
mod tests {
    use super::TimeOutput;

    #[test]
    fn as_time_output() {
        let pairs = vec![
            (5_u64, "0:00:05"),
            (60_u64, "0:01:00"),
            (154_u64, "0:02:34"),
            (3606_u64, "1:00:06"),
        ];
        for (input, output) in pairs {
            assert_eq!(input.as_time().as_str(), output);
        }
    }
}
