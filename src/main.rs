mod term;
mod variable;
mod sum;
mod product;
mod fraction;
mod power;
mod root;
mod input;
mod math;

use std::{io::{self, stdout, Write}};

use crate::{input::*};

fn main() {
	println!("Terminal calculator v0.1.0\nWrite 'exit' to exit");
	loop
	{
		print!("> ");
		stdout().flush().expect("Error flushing stdout");
		let mut input: String = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read input");
		if input.starts_with("exit")
		{
			println!("Exiting.");
			break;
		}
		match term_from_string(&input)
		{
			Ok(term) => {
				let calculated_term = term.calculate(false);
				println!("  {}\n= {}\n≈ {}", term, calculated_term, calculated_term.calculate(true))
			},
			Err(e) => println!("INPUT ERROR: {}", e)
		}
	}
}