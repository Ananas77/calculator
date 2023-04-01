mod term;
mod variable;
mod sum;
mod product;

use crate::term::*;
use crate::variable::*;
use crate::sum::*;
use crate::product::*;

fn main() {
	let num_1 = Box::new(Number::new(3.0)) as Box<dyn Term>;
	let num_2 = Box::new(Number::new(2.0)) as Box<dyn Term>;
	let num_3 = Box::new(Number::new(5.0)) as Box<dyn Term>;
	let var_1 = Box::new(Variable::new("a")) as Box<dyn Term>;
	let sum_1 = Box::new(Sum::new(vec![num_2, var_1.copy()])) as Box<dyn Term>;
	let sum_2 = Box::new(Sum::new(vec![num_3, var_1])) as Box<dyn Term>;
	let my_term:Box<dyn Term> = Box::new(Product::new(vec![num_1, sum_1.copy(), sum_2]));
	println!("{} = {}", my_term, my_term.calculate());
}