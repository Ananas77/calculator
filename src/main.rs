mod term;
mod variable;
mod sum;
mod product;
mod fraction;
mod power;
mod input;
mod math;

use std::io;

use crate::{input::*};

fn main() {
	loop
	{
		let mut input: String = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read input");
		if input.starts_with("exit")
		{
			break;
		}
		let term = term_from_string(input);
		println!("{}", term, /* term.calculate(false) */);
	}
}