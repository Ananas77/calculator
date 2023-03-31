use crate::term::*;

pub struct Sum
{
	summands: Vec<Box<dyn Term>>
}

impl Term for Sum
{
	fn calculate(&self) -> Box<dyn Term> {
		// calculate result
		let result:Box<dyn Term>;
		let mut new_summands:Vec<Box<dyn Term>> = vec![];
		let mut number_result = Number::new(0.0);
		for term in &self.summands
		{
			let calculated_term = term.calculate();
			match calculated_term.get_type()
			{
				TermType::Number => number_result = Number::new(number_result.get_value() + calculated_term.get_value()),
				_ => new_summands.push(calculated_term)
			}
		};
		
		// format result
		if number_result.get_value() != 0.0
		{
			new_summands.push(Box::new(number_result));
		}
		match new_summands.len()
		{
			1 => result = new_summands.remove(0),
			0 => result = Box::new(Number::new(0.0)),
			_ => result = Box::new(Sum::new(new_summands)),
		}
		result
	}

	fn print(&self) -> String {
		if self.summands.len() != 0
		{
			let mut result = format!("{}", self.summands[0].print());
			for summand in &self.summands[1..]
			{
				result = format!("{} + {}", result, summand.print());
			}
			result
		}
		else 
		{
			"".to_string()    
		}        
	}

	fn get_parts(&self) -> Vec<Box<dyn Term>> {
		let mut result = vec![];
		for summand in &self.summands
		{
			result.push(summand.copy())
		}
		result
	}

	fn get_type(&self) -> TermType {
		TermType::Sum
	}

	fn copy(&self) -> Box<dyn Term> {
		let mut summand_copy = vec![];
		for summand in &self.summands
		{
			summand_copy.push(summand.copy());
		}
		Box::new(Sum::new(summand_copy))
	}
}

impl Sum
{
	pub fn new(summands:Vec<Box<dyn Term>>) -> Sum
	{
		Sum
		{
			summands:summands
		}
	}
}
