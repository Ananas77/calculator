use std::collections::HashMap;
use std::vec;

use crate::fraction::Fraction;
use crate::power::Power;
use crate::term::*;
use crate::sum::*;

pub struct Product
{
	factors: Vec<Box<dyn Term>>
}

impl Term for Product
{
	fn calculate(&self, round: bool) -> Box<dyn Term> {
		// calculate result
		let mut result:Box<dyn Term>; // this will later be returned
		let mut calculated_factors: Vec<Box<dyn Term>> = vec![];
		let mut new_factors:Vec<Box<dyn Term>> = vec![];	// the factors that remain from the original product
		let mut number_result = Number::new(1.0);	// any numbers get multiplied directly with this value
		let mut variables: Vec<Box<dyn Term>> = vec![];	// variables get in here to be calculated and sorted later
		let mut fractions: Vec<Box<dyn Term>> = vec![];	// fractions get in here to be calculated later
		
		// calculate factors
		for term in &self.factors
		{
			let calculated_term = term.calculate(round);
			if calculated_term.get_type() == TermType::Error
			{
				return calculated_term
			}
			if TermType::Product == calculated_term.get_type()
			{
				calculated_factors.extend(calculated_term.get_parts());
			}
			else
			{
				calculated_factors.push(calculated_term);
			}
		}

		// goes through every factor and tries to calculate anything that can be calculated
		for term in &calculated_factors
		{
			match term.get_type()
			{
				TermType::Number => number_result = Number::new(number_result.get_value() * term.get_value()),
				TermType::Variable => variables.push(term.copy()),
				TermType::Product => new_factors.extend(term.get_parts()),
				TermType::Fraction => fractions.push(term.copy()),
				_ => new_factors.push(term.copy())
			};
		};

		// add the number result to the new factors
		if number_result.get_value() == 0.0
		{
			result = Box::new(Number::new(0.0));
			return result;
		};
		if number_result.get_value() != 1.0
		{
			new_factors.push(Box::new(number_result));
		};

		// sort the variables
		variables.sort_by(|a, b| a.print().to_lowercase().cmp(&b.print().to_lowercase()));
		// add the variables to the new factors
		new_factors.extend(variables);

		// calculate powers
		let mut factors_as_hash: HashMap<Box<dyn Term>, Box<dyn Term>> = HashMap::new();
		for factor in &new_factors
		{
			if factor.get_type() == TermType::Power
			{
				if let Some(exponent) = factors_as_hash.get_mut(&factor.get_parts()[0])
				{
					*exponent = Sum::new(vec![exponent.copy(), factor.get_parts()[1].copy()]).calculate(round)
				}
				else {
					factors_as_hash.insert(factor.get_parts()[0].copy(), factor.get_parts()[1].copy());
				}
			}
			else {
				if let Some(exponent) = factors_as_hash.get_mut(&factor.copy())
				{
					*exponent = Sum::new(vec![exponent.copy(), Box::new(Number::new(1.0))]).calculate(round)
				}
				else {
					factors_as_hash.insert(factor.copy(), Box::new(Number::new(1.0)));
				}
			}
		}

		new_factors = Vec::new();

		for (key, value) in factors_as_hash.into_iter()
		{
			if value.get_type() == TermType::Number && value.get_value() == 1.0
			{
				new_factors.extend(vec![key; (value.get_value() as i64).try_into().unwrap()]);
			}
			else
			{
				new_factors.push(Box::new(Power::new(key, value)) as Box<dyn Term>)
			}
		}
		new_factors.sort_by(|a, b| a.print().to_lowercase().cmp(&b.print().to_lowercase()));

		// format the new factors
		match new_factors.len()
		{
			1 => result = (&new_factors[0]).copy(),
			0 => result = Box::new(Number::new(1.0)),
			_ => result = 
			{
				Box::new(Product::new(new_factors.iter().map(|factor| factor.copy()).collect()))
			},
		}

		// check for sums and multiply their summands with the remaining product
		for term in &new_factors
		{
			if term.get_type() == TermType::Sum
			{
				let mut summands:Vec<Box<dyn Term>> = vec![];
				let mut left_factors: Vec<Box<dyn Term>> = new_factors.iter().map(|f| f.copy()).collect();
				left_factors.retain(|factor| factor != term);
				for summand in term.get_parts()
				{
					let mut factors  = vec![summand];
					factors.extend(left_factors.iter().map(|f| f.copy()));
					summands.push(Box::new(Product::new(factors)));
				}
		// calculate the resulting sum
				result = Sum::new(summands).calculate(round);
				return result;
			}
		};

		// calculate fractions
		if fractions.len() > 0
		{
			let mut numerators = vec![];
			let mut denominators = vec![];
			// calculate fractions
			for fraction in fractions
			{
				let parts = fraction.get_parts();
				numerators.push(parts[0].copy());
				denominators.push(parts[1].copy());
			}
			numerators.push(result);
			result = Fraction::new(Box::new(Product::new(numerators)), Box::new(Product::new(denominators))).calculate(round);
		}

		// return
		result
	}

	fn print(&self) -> String {
		let mut result = "".to_string(); // this will later be returned
		let mut i = 0; // index for the following for loop
		
		// add each factor to result string
		for factor in &self.factors
		{
			if i != 0 && factor.get_type() != TermType::Variable && factor.get_type() != TermType::Sum && !(i > 0 && self.factors[i-1].get_type() == TermType::Number && self.factors[i-1].get_value() == -1.0)
			{
				result = format!("{} * ", result);
			}
			i += 1;
			match factor.get_type()
			{
				// Sums and negative numbers require parenthesis
				TermType::Sum | TermType::Fraction | TermType::Power => result = format!("{}({})", result, factor.print()),
				TermType::Number => {
					if factor.get_value() < 0.0 && factor.get_value() != -1.0
					{
						result = format!("{}({})", result, factor.print())
					}
					else if factor.get_value() == -1.0
					{
						result.insert(0, '-')
					}
					else 
					{
						result = format!("{}{}", result, factor.print())
					}
				}
				_ => result = format!("{}{}", result, factor.print())
			}
		}

		// return
		result
	}

	fn get_parts(&self) -> Vec<Box<dyn Term>> // returns the product's factors
	{
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
		Box::new(Product::new(factor_copy))
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
