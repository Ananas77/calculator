mod term;
mod variable;
mod sum;
mod product;
mod fraction;
mod power;
mod root;
mod input;
mod math;

use std::io::{self, stdout, Write};

use crate::input::*;

fn main() {
	println!("Terminal calculator v0.1.0");
	println!("Use 'help' for more information.");
	loop
	{
		print!("> ");
		stdout().flush().expect("Error flushing stdout");
		let mut input: String = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read input");
		if input == "exit\n"
		{
			println!("Exiting.");
			break;
		}
		else if input == "clear\n"
		{
			print!("\x1B[2J\x1B[1;1H");
		}
		else if input == "help\n"
		{
			println!("Available commands:");
			println!("  exit - Exits the program");
			println!("  clear - Clears the screen");
			println!("  help - Prints this message");
			println!("");
			println!("  <term> - Evaluates the term and prints the result");
		}
		else {
			match term_from_string(&input)
			{
				Ok(term) => {
					let calculated_term = term.calculate(false);
					println!("  {}\n= {}", term, calculated_term);
					println!("≈ {}", calculated_term.calculate(true))
				},
				Err(e) => println!("INPUT ERROR: {}", e)
			}
		}
	}
}