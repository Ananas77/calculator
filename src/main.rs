mod term;
mod variable;
mod sum;

use crate::term::*;
use crate::variable::*;
use crate::sum::*;

fn main() {
	let my_term:Box<dyn Term> = Box::new(Sum::new(vec![Box::new(Number::new(3.0)), Box::new(Number::new(-1.3)), Box::new(Variable::new("a"))]));
	println!("{}", my_term.calculate());
}