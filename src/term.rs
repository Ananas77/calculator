use std::fmt;

#[derive(PartialEq)]
pub enum TermType
{
	None,
	Number,
	Sum,
}

pub trait Term
{
	fn calculate(&self) -> Box<dyn Term> {Box::new(Number::new(0.0))} 	// returns the term's exact value
	fn print(&self) -> String {"0".to_string()}                   		// prints the term
	fn get_value(&self) -> f32 {0.0} 									// returns an exact value as a float if possible
	fn get_type(&self) -> TermType {TermType::None}
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
	fn calculate(&self) -> Box<dyn Term>
	{
		Box::new(Number{value:self.value})
	}

	fn print(&self) -> String
	{
		self.value.to_string()
	}

	fn get_value(&self) -> f32
	{
		self.value
	}

	fn get_type(&self) -> TermType {
		TermType::Number
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