use std::{env, fs};
use std::io::{self, Write, stdout, Error, ErrorKind};
use std::process::Command;

mod builtins;

fn check_paths(input_buffer: &str) -> Result<String, io::Error> {
	let paths = env::var("PATH").expect("could not open PATH");

	for path in paths.split(":") {
		let program_str = format!("{}/{}", path, input_buffer);

		let metadata = fs::metadata(&program_str);

		if metadata.is_ok() {
			return Ok(program_str);
		}
	}

	Err(Error::new(ErrorKind::NotFound, "unknown command"))
}

fn main() {
	loop
	{
		print!("$>");	
		stdout().flush().expect("Could not flush output");

		let mut input_buffer = String::new();
		let stdin = io::stdin();

		stdin.read_line(&mut input_buffer).expect("Could not read input");

		input_buffer.truncate(input_buffer.len() - 1);

		let arg_index = input_buffer.find(char::is_whitespace);
		let mut args: String = "".to_string();

		if arg_index.is_some() {
			args = input_buffer.split_off(arg_index.unwrap());
		}

		match input_buffer.as_str() {
			"cd" => builtins::change_directory(args),
			"export" => builtins::set_environment(&args),
			"unset" => builtins::unset_environment(&args),
			"echo" => {
				let command_path = check_paths(&input_buffer);

				if command_path.is_ok() {
					let mut child = Command::new(input_buffer);				

					if !args.is_empty() {
						child.args(args.split_whitespace());
					}

					child.status().expect("unknown command");
				} else {
					builtins::echo_builtin(&args);
				}
			},
			_ => {
				let command_path = check_paths(&input_buffer);

				if command_path.is_ok() {
					let mut child = Command::new(input_buffer);				

					if !args.is_empty() {
						child.args(args.split_whitespace());
					}

					child.status().expect("unknown command");
				} else {
					println!("unknown command");
				}
			}

		}
	}
}
