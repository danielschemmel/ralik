pub fn print_error_chain(error: &dyn std::error::Error) {
	write_error_chain(std::io::stdout().lock(), error).unwrap()
}

pub fn write_error_chain(mut writer: impl std::io::Write, error: &dyn std::error::Error) -> std::io::Result<()> {
	println!("Error: {}", error);
	let mut source = error.source();
	while let Some(err) = source {
		writeln!(writer, "Caused by: {}", err)?;
		source = err.source();
	}

	Ok(())
}
