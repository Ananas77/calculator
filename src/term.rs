use std::{fmt, vec};

use crate::fraction::Fraction;

#[derive(PartialEq, Clone, Copy)]
pub enum TermType
{
	None,
	Number,
	Variable,
	Sum,
	Product,
	Fraction,
	Power,
}

pub trait Term
{
	fn calculate(&self, _: bool) -> Box<dyn Term> {panic!("Trying to calculate empty term")}	// returns the term's exact value
	fn print(&self) -> String {panic!("Trying to get value of an empty term")}	// returns the term as string
	fn get_value(&self) -> f64 {panic!("Trying to get value of an empty term")}	// returns an exact value as a float if possible
	fn get_type(&self) -> TermType {TermType::None}	// returns the term's type (also see enum TermType)
	fn get_parts(&self) -> Vec<Box<dyn Term>> {vec![self.copy()]}	// returns the summands or factors of a term
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
	value:f64
}

impl Term for Number
{
	// returns value
	fn calculate(&self, rounded: bool) -> Box<dyn Term>
	{
		if self.value.round() == self.value || rounded
		{
			Box::new(Number{value:(self.value * 10000000000.0).round() / 10000000000.0})
		}
		else {
			Fraction::new(Box::new(Number::new((self.value * 10000000000.0).round())), Box::new(Number::new(10000000000.0))).calculate(rounded)
		}
	}

	fn print(&self) -> String
	{
		self.value.to_string()
	}

	fn get_value(&self) -> f64
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
	pub fn new(value:f64) -> Self
	{
		Number
		{
			value:value,
		}
	}
}