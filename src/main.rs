use unicode_width::UnicodeWidthStr;
use std::io::Write;

use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};

fn main() {
	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	let sink = Sink::try_new(&stream_handle).unwrap();

	let mut stdout = std::io::stdout().lock();

	for line in std::io::stdin().lines() {
		let line: String = line
			.unwrap()
			.chars()
			.map(|c| if c == '\t' { String::from(" ").repeat(8) } else { c.to_string() })
			.collect();

		let len = UnicodeWidthStr::width(&*line) as f32;

		stdout.write_fmt(format_args!("{}\n", line)).unwrap();

		sink.append(SineWave::new(44.0 * len)
			.take_duration(Duration::from_secs_f32(0.1))
			.amplify(0.20));
		sink.sleep_until_end();
	}
}
