use anyhow::Result;

#[derive(structopt::StructOpt, Debug)]
#[structopt(
	setting = structopt::clap::AppSettings::ColoredHelp,
	version = build_info::format!("{} {}\n\nBuilt from {} at {} with {} for {} on {}.", $.crate_info.version, $.profile, $.version_control, $.timestamp, $.compiler, $.compiler.target_triple, $.compiler.host_triple),
)]
pub struct Args {}

#[derive(Copy, Clone)]
pub enum ReturnCode {
	Success = 0,
	CtrlC = -1,
	ArgumentParsing = 1,
}

fn set_ctrlc_handler() -> Result<std::sync::mpsc::Receiver<()>> {
	use std::sync::atomic::{AtomicBool, Ordering};
	use std::sync::Arc;

	let (sender, receiver) = std::sync::mpsc::sync_channel(1);
	let previous_ctrlc = Arc::new(AtomicBool::new(false));

	ctrlc::set_handler(move || {
		if (*previous_ctrlc).swap(true, Ordering::Relaxed) {
			eprintln!("\nReceived Ctrl+C again: Terminating forcefully!");
			std::process::exit(ReturnCode::CtrlC as i32);
		} else {
			eprintln!("\nReceived Ctrl+C...");
			sender
				.send(())
				.map_err(|e| eprintln!("Could not notify main program: {}", e))
				.ok();
		}
	})?;

	Ok(receiver)
}

const PROMPT: &str = "> ";

pub fn main(args: Args) -> Result<ReturnCode> {
	let _ctrlc = set_ctrlc_handler()?;

	println!("{:?}", args);

	use std::io::BufRead;
	use std::io::Write;
	let stdin = std::io::stdin();
	let stdin = stdin.lock();
	let stdout = std::io::stdout();
	print!("{}", PROMPT);
	stdout.lock().flush().unwrap();
	for line in stdin.lines() {
		match ralik::run_str(&line.unwrap()) {
			Ok(expr) => {
				println!("{:+#?}", expr);
			}
			Err(err) => {
				print_error_chain(&err);
			}
		}
		print!("{}", PROMPT);
		stdout.lock().flush().unwrap();
	}

	if atty::is(atty::Stream::Stdin) {
		println!("exit()");
	}

	Ok(ReturnCode::Success)
}

fn print_error_chain(error: &dyn std::error::Error) {
	write_error_chain(std::io::stdout().lock(), error).unwrap()
}

fn write_error_chain<W: std::io::Write>(mut writer: W, error: &dyn std::error::Error) -> std::io::Result<()> {
	println!("{}", error);
	let mut source = error.source();
	while let Some(err) = source {
		writeln!(writer, "Caused by: {}", err)?;
		source = err.source();
	}

	Ok(())
}
