use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum TermType
{
	None,
	Number,
	Variable,
	Sum,
	Product,
}

pub trait Term
{
	fn calculate(&self) -> Box<dyn Term> {panic!("Trying to calculate empty term")}	// returns the term's exact value
	fn print(&self) -> String {panic!("Trying to get value of an empty term")}	// returns the term as string
	fn get_value(&self) -> f32 {panic!("Trying to get value of an empty term")}	// returns an exact value as a float if possible
	fn get_type(&self) -> TermType {TermType::None}	// returns the term's type (also see enum TermType)
	fn get_parts(&self) -> Vec<Box<dyn Term>> {panic!("Trying to get parts of an empty term")}	// returns the summands or factors of a term
	fn copy(&self) -> Box<dyn Term> {panic!("Trying to copy an empty Term")}	// returns a term with the same values (alternative to implementing the 'Copy' trait, which is not possible)
}

impl fmt::Display for dyn Term // for printing terms
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.print())
	}
}

#[derive(Clone, Copy)]
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

	fn copy(&self) -> Box<dyn Term> {
		Box::new(self.clone())
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