use std::{collections::HashMap, vec};

use crate::{term::*, product::Product};

pub struct Sum
{
	summands: Vec<Box<dyn Term>>
}

impl Term for Sum
{
	fn calculate(&self) -> Box<dyn Term> {
		// calculate result
		let result:Box<dyn Term>;	// this will later be returned
		let mut calculated_summands: Vec<Box<dyn Term>> = vec![];
		let mut new_summands:Vec<Box<dyn Term>> = vec![];	// the summands that remain from the original sum
		let mut number_result = Number::new(0.0);	// any numbers get added directly to this value
		let mut products: Vec<Box<dyn Term>> = vec![];	// products are put in this vector to be calculated later

		// calculate summands
		for term in &self.summands
		{
			let calculated_term = term.calculate();
			if TermType::Sum == calculated_term.get_type()
			{
				calculated_summands.extend(calculated_term.get_parts());
			}
			else
			{
				calculated_summands.push(calculated_term);
			}
		}

		// go through each summand and try to add it to the rest
		for term in calculated_summands
		{
			match term.get_type()
			{
				TermType::Number => number_result = Number::new(number_result.get_value() + term.get_value()),
				TermType::Sum => new_summands.extend(term.get_parts()),
				TermType::Product => products.push(term),
				_ => new_summands.push(term)
			}
		};
		
		// add products
		let mut new_products: Vec<Box<dyn Term>> = vec![];	// a list with the added products
		for product in &products	// go through each found product and try to find a product in new_products to add it to
		{
			let mut factors: Vec<Box<dyn Term>> = vec![];	// the products non-number factors
			let mut factors_as_hash = HashMap::new();	// factors as a hash map (for later comparison)
			let mut quantity = 1.0;	// the number factors (for later addition)
			for factor in &product.get_parts()	// add product factors to the three variables above
			{
				match factor.get_type() {
					TermType::Number => quantity = factor.get_value(),
					_ => 
					{
						factors.push(factor.copy());
						*factors_as_hash.entry(factor.print()).or_insert(0) += 1;
					}
				}
			}
			let mut add = true;	// to check wether the current product should be added to new_products
			// the same as before but for each product in new_products
			let mut other_factors: Vec<Box<dyn Term>> = vec![];
			let mut other_quantity = 1.0;
			let mut i = 0;
			for other_product in &new_products
			{
				let mut other_factors_as_hash = HashMap::new();
				for factor in &other_product.get_parts()
				{
					match factor.get_type() {
						TermType::Number => other_quantity = factor.get_value(),
						_ => 
						{
							other_factors.push(factor.copy());
							*other_factors_as_hash.entry(factor.print()).or_insert(0) += 1;
						}
					}
				}
				// check if the non-number factors are equal
				if factors_as_hash == other_factors_as_hash
				{
					add = false;
					break;
				}
				i += 1
			}
			if add
			{
				new_products.push(product.copy());
			}
			else {
				factors.insert(0, Box::new(Number::new(quantity + other_quantity)));	// add the number factors, multiply the result with the non-number factors
				new_products.remove(i);	// remove the old element
				new_products.push(Product::new(factors).calculate());	// add the new one
			}
		}

		// add the new products to the summands
		new_summands.extend(new_products);

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

		// return
		result
	}

	fn print(&self) -> String {
		if self.summands.len() != 0
		{
			let mut result = "".to_string();
			let mut i = 0; // index for the following for loop
			
			// add each factor to the result string
			for summand in &self.summands
			{
				if i != 0
				{
					result = format!("{} + ", result);
				}
				i += 1;
				match summand.get_type()
				{
					// Negative numbers require parentheses
					TermType::Number => {
						if summand.get_value() < 0.0
						{
							result = format!("{}({})", result, summand.print())
						}
						else 
						{
							result = format!("{}{}", result, summand.print())
						}
					}
					_ => result = format!("{}{}", result, summand.print())
				}
			}

			// return
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
