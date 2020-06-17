use anyhow::Result;

#[derive(structopt::StructOpt, Debug)]
#[structopt(
	setting = structopt::clap::AppSettings::ColoredHelp,
	version = build_info::format!("{} {}\n\nBuilt from {} at {} with {} for {} on {}.", $.crate_info.version, $.profile, $.version_control, $.timestamp, $.compiler, $.compiler.target_triple, $.compiler.host_triple),
)]
pub enum Args {
	Repl(crate::interpreter::repl::Args),
	Run(crate::interpreter::run::Args),
	Stats(crate::stats::Args),
}

#[derive(Copy, Clone)]
pub enum ReturnCode {
	Success = 0,
	CtrlC = -1,
	ArgumentParsing = 1,
	ScriptError = 2,
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

pub fn main(args: Args) -> Result<ReturnCode> {
	set_ctrlc_handler()?; // only active when not replaced by rustyline

	match args {
		Args::Run(args) => crate::interpreter::run::main(args),
		Args::Repl(args) => crate::interpreter::repl::main(args),
		Args::Stats(args) => crate::stats::main(args),
	}
}
