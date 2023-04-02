mod term;
mod variable;
mod sum;
mod product;
mod fraction;
mod input;
mod math;

use std::io;

use crate::{input::*};

fn main() {
	let mut input: String = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read input");
	let term = term_from_string(input);
	println!("{} = {}", term, term.calculate(false));
}