use mown::Mown;
use std::io::{self, Read};

fn response(input: &str) -> Mown<str> {
	if input == "Hello\n" {
		Mown::Owned("World!".to_string())
	} else {
		Mown::Borrowed(input)
	}
}

fn main() -> io::Result<()> {
	let mut buffer = String::new();
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	handle.read_to_string(&mut buffer)?;

	let output = response(buffer.as_ref());
	println!("{}", output);

	Ok(())
}
