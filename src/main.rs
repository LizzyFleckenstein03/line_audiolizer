use std::{io::{self, Write}, time::Duration};
use unicode_width::UnicodeWidthStr;
use rodio::{OutputStream, Sink, source::{SineWave, Source}};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
	/// How many spaces a tab character counts as
	#[clap(short, long, value_parser, default_value_t = 4)]
	tab_width: usize,

	/// Frequency of tones. Multiplied by line length
	#[clap(short, long, value_parser, default_value_t = 20.0)]
	frequency: f32,

	/// Duration of each tone, in seconds
	#[clap(short, long, value_parser, default_value_t = 0.1)]
	duration: f32,

	/// Amplifier of tones
	#[clap(short, long, value_parser, default_value_t = 1.0)]
	amplifier: f32,
}

fn main() {
	let args = Args::parse();

	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	let sink = Sink::try_new(&stream_handle).unwrap();

	let mut stdout = io::stdout().lock();

	for line in io::stdin().lines() {
		let line: String = line
			.unwrap()
			.chars()
			.map(|c|
				if c == '\t' {
					String::from(" ").repeat(args.tab_width)
				} else {
					c.to_string()
				}
			)
			.collect();

		let len = UnicodeWidthStr::width(&*line) as f32;

		stdout.write_fmt(format_args!("{}\n", line)).unwrap();

		sink.append(SineWave::new(args.frequency * len)
			.take_duration(Duration::from_secs_f32(args.duration))
			.amplify(args.amplifier));
		sink.sleep_until_end();
	}
}
