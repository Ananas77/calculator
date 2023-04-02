mod term;
mod variable;
mod sum;
mod product;
mod fraction;
mod input;
mod math;

use std::io;

use math::prime_factors;
use term::Number;

use crate::{input::*, term::Term};

fn main() {
	let mut input: String = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read input");
	let term = term_from_string(input);
	println!("{} = {}", term, term.calculate());
	println!("{}, {}", prime_factors(Box::new(Number::new(20.0))).print(), Box::new(Number::new(2.0)).print());
}