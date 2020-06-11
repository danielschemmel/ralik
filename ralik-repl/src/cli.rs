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
	let (sender, receiver) = std::sync::mpsc::sync_channel(1);

	ctrlc::set_handler(move || match sender.try_send(()) {
		Ok(()) => {
			eprintln!("\nReceived Ctrl+C...");
		}
		Err(std::sync::mpsc::TrySendError::Full(())) => {
			eprintln!("\nReceived Ctrl+C again: Terminating forcefully!");
			std::process::exit(ReturnCode::CtrlC as i32);
		}
		Err(std::sync::mpsc::TrySendError::Disconnected(())) => {
			eprintln!("\nReceived Ctrl+C. Terminating now.");
			std::process::exit(ReturnCode::CtrlC as i32);
		}
	})?;

	Ok(receiver)
}

const PROMPT: &str = "> ";

pub fn main(args: Args) -> Result<ReturnCode> {
	set_ctrlc_handler()?;

	println!("{:?}", args);
	let context = ralik::Context::new();

	use std::io::BufRead;
	use std::io::Write;
	let stdin = std::io::stdin();
	let stdin = stdin.lock();
	let stdout = std::io::stdout();
	print!("{}", PROMPT);
	stdout.lock().flush().unwrap();
	for line in stdin.lines() {
		match ralik::eval_str(&line.unwrap(), &context) {
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
