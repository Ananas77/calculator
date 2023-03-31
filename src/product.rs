use crate::term::*;
use crate::sum::*;

pub struct Product
{
	factors: Vec<Box<dyn Term>>
}

impl Term for Product
{
	fn calculate(&self) -> Box<dyn Term> {
		// calculate result
		let mut result:Box<dyn Term>;
		let mut new_factors:Vec<Box<dyn Term>> = vec![];
		let mut number_result = Number::new(1.0);
		let mut i = 0;
		for term in &self.factors
		{
			let calculated_term = term.calculate();
			match calculated_term.get_type()
			{
				TermType::Number => number_result = Number::new(number_result.get_value() * calculated_term.get_value()),
				_ => new_factors.push(calculated_term)
			};
			i += 1;
		};
		if number_result.get_value() != 1.0
		{
			new_factors.push(Box::new(number_result));
		};

		match new_factors.len()
		{
			1 => result = (&new_factors[0]).copy(),
			0 => result = Box::new(Number::new(1.0)),
			_ => result = 
			{
				let mut new_factors_copied = vec![];
				for factor in &new_factors
				{
					new_factors_copied.push(factor.copy());
				}
				Box::new(Product::new(new_factors_copied))
			},
		}

		i = 0;
		for term in &new_factors
		{
			if term.get_type() == TermType::Sum
			{
				let mut summands:Vec<Box<dyn Term>> = vec![];
				for summand in term.get_parts()
				{
					let mut factors: Vec<Box<dyn Term>> = vec![];
					let mut j = 0;
					for factor in &new_factors
					{
						if i != j
						{
							factors.push(factor.copy());
						}
						else
						{
							factors.push(summand.copy());	
						}
						j += 1;
					}
					summands.push(Box::new(Product::new(factors)));
				}
				result = Sum::new(summands).calculate();
				break;
			}
			i+=1;
		};
		result
	}

	fn print(&self) -> String {
		let mut result = "".to_string();
		let mut i = 0;
		for factor in &self.factors
		{
			if i != 0
			{
				result = format!("{} *", result);
			}
			i += 1;
			match factor.get_type()
			{
				TermType::Sum => result = format!("{} ({})", result, factor.print()),
				TermType::Number => {
					if factor.get_value() < 0.0
					{
						result = format!("{} ({})", result, factor.print())
					}
					else 
					{
						result = format!("{} {}", result, factor.print())
					}
				}
				_ => result = format!("{} {}", result, factor.print())
			}
		}
		result
	}

	fn get_parts(&self) -> Vec<Box<dyn Term>> {
		let mut result = vec![];
		for factor in &self.factors
		{
			result.push(factor.copy())
		}
		result
	}

	fn get_type(&self) -> TermType {
		TermType::Product
	}

	fn copy(&self) -> Box<dyn Term> {
		let mut factor_copy = vec![];
		for factor in &self.factors
		{
			factor_copy.push(factor.copy());
		}
		Box::new(Sum::new(factor_copy))
	}
}

impl Product
{
	pub fn new(factors:Vec<Box<dyn Term>>) -> Product
	{
		Product
		{
			factors:factors
		}
	}
}
