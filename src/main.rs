mod term;
use crate::term::*;

fn main() {
	let my_number:Box<dyn Term> = Box::new(Number::new(3.0));
	println!("hi {}", my_number.solve());
}