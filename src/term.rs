use std::fmt;

pub trait Term
{
	fn solve(&self) -> Box<dyn Term> {Box::new(Number::new(0.0))} // returns the term's exact value
	fn print(&self) -> String {"0".to_string()}                   // print the term
}

impl fmt::Display for dyn Term {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.print())
	}
}

pub struct Number
{
	value:f32
}

impl Term for Number
{
	// returns value
	fn solve(&self) -> Box<dyn Term>
	{
		Box::new(Number{value:self.value})
	}

	fn print(&self) -> String
	{
		self.value.to_string()
	}
}

impl Number
{
	pub fn new(value:f32) -> Self
	{
		Number
		{
			value:value,
		}
	}
}