use std::{vec};

use crate::{term::*, math::prime_factors, product::Product, sum::Sum};

pub struct Fraction
{
	numerator: Box<dyn Term>,
	denominator: Box<dyn Term>
}

impl Term for Fraction
{
	fn calculate(&self, round: bool) -> Box<dyn Term> {
		let result: Box<dyn Term>;  // will later be returned
		let mut fraction_is_neg = 1.0;
		// calculate numerator and denominator
		let mut calculated_numerator = self.numerator.calculate(round);
		let mut calculated_denominator = self.denominator.calculate(round);
		match calculated_numerator.get_type()
		{
			TermType::Fraction => {
				calculated_denominator = Product::new(vec![calculated_numerator.get_parts()[1].copy(), calculated_denominator]).calculate(round);
				calculated_numerator = calculated_numerator.get_parts()[0].copy()
			}
			_ => {}
		}
		match calculated_denominator.get_type()
		{
			TermType::Fraction => {
				calculated_numerator = Product::new(vec![calculated_denominator.get_parts()[1].copy(), calculated_numerator]).calculate(round);
				calculated_denominator = calculated_denominator.get_parts()[0].copy()
			}
			_ => {}
		}
		if calculated_numerator.get_type() == TermType::Sum
		{
			return Box::new(Sum::new(calculated_numerator.get_parts().iter().map(|summand| Box::new(Fraction::new(summand.copy(), calculated_denominator.copy())) as Box<dyn Term>).collect()))
		}
		if calculated_denominator.get_type() == TermType::Sum
		{
			return Box::new(Sum::new(calculated_denominator.get_parts().iter().map(|summand| Box::new(Fraction::new(calculated_numerator.copy(), summand.copy())) as Box<dyn Term>).collect()))
		}
		// reduce the fraction
		let prime_factors_numerator = prime_factors(calculated_numerator).get_parts();    // get numerator and denominator into products, reduce the fraction
		let prime_factors_denominator = prime_factors(calculated_denominator).get_parts();
		let mut new_factors_numerator: Vec<Box<dyn Term>> = vec![]; // what is left from the numerator after reducing
		let mut new_factors_denominator: Vec<Box<dyn Term>> = vec![];   // what is left from the denominator after reducing
		// go through each prime factor in the numerator and check wether the fraction can be reduced by it
		for factor in &prime_factors_denominator
		{
			if factor.get_type() == TermType::Number && factor.get_value() < 0.0
			{
				fraction_is_neg *= -1.0;
			}
			else {
				new_factors_denominator.push(factor.copy());
			}
		}
		for factor_numerator in &prime_factors_numerator
		{
			let mut reduced_by_factor = false;
			let mut i = 0;
			for factor_denominator in &new_factors_denominator
			{
				if factor_numerator == factor_denominator
				{
					reduced_by_factor = true;
					new_factors_denominator.remove(i);
					break;
				}
				i += 1;
			}
			if !reduced_by_factor {
				if factor_numerator.get_type() == TermType::Number && factor_numerator.get_value() < 0.0
				{
					fraction_is_neg *= -1.0;
				}
				else {
					new_factors_numerator.push(factor_numerator.copy());
				}
			}
		}

		// calculate the resulting fraction
		new_factors_numerator.insert(0, Box::new(Number::new(fraction_is_neg)));
		let new_numerator = Product::new(new_factors_numerator).calculate(round);
		result = match new_factors_denominator.len()
		{
			0 => new_numerator,    // check wether the result is a fraction
			_ =>
			{
				let new_denominator = Product::new(new_factors_denominator).calculate(round);
				if round
				{
					let numerator_factors = prime_factors(new_numerator.copy()).get_parts();
					let denominator_factors = prime_factors(new_denominator.copy()).get_parts();
					let mut new_numerator_factors = vec![];
					let mut new_denominator_factors = vec![];
					let mut numerator_coefficient = 1.0;
					let mut denominator_coefficient = 1.0;
					// get all numbers from the numerator and denominator
					for factor in &numerator_factors
					{
						match factor.get_type()
						{
							TermType::Number => {
								numerator_coefficient *= factor.get_value();
							},
							_ => {
								new_numerator_factors.push(factor.copy());
							}
						}
					};
					for factor in &denominator_factors
					{
						match factor.get_type()
						{
							TermType::Number => {
								denominator_coefficient *= factor.get_value();
							},
							_ => {
								new_denominator_factors.push(factor.copy());
							}
						}
					};
					let fraction_coefficient = numerator_coefficient / denominator_coefficient;
					// format
					if fraction_coefficient != 1.0
					{
						if new_numerator_factors.len() > 0
						{
							if new_denominator_factors.len() > 0
							{
								println!("{}",denominator_factors[0]);
								Box::new(Product::new(vec![Box::new(Number::new(fraction_coefficient)), Box::new(Fraction::new(Product::new(new_numerator_factors).calculate(round), Box::new(Product::new(new_denominator_factors))))]))
							}
							else
							{
								Box::new(Product::new(vec![Box::new(Number::new(fraction_coefficient)), Product::new(new_numerator_factors).calculate(round)]))
							}
						}
						else
						{
							if new_denominator_factors.len() > 0
							{
								Box::new(Product::new(vec![Box::new(Number::new(fraction_coefficient)), Box::new(Fraction::new(Box::new(Number::new(1.0)), Box::new(Product::new(new_denominator_factors))))]))
							}
							else
							{
								Box::new(Number::new(fraction_coefficient))
							}
						}
					}
					else 
					{
						if new_numerator_factors.len() > 0
						{
							if new_denominator_factors.len() > 0
							{
								Box::new(Fraction::new(Product::new(new_numerator_factors).calculate(round), Box::new(Product::new(new_denominator_factors))))
							}
							else
							{
								Product::new(new_numerator_factors).calculate(round)
							}
						}
						else
						{
							if new_denominator_factors.len() > 0
							{
								Box::new(Fraction::new(Box::new(Number::new(1.0)), Box::new(Product::new(new_denominator_factors))))
							}
							else
							{
								Box::new(Number::new(1.0))
							}
						}
					}
					
				}
				else {
					Box::new(Fraction::new(new_numerator, new_denominator))
				}
			}
		};

		result
	}

	fn print(&self) -> String
	{
		let mut result = String::new();
		match self.numerator.get_type()
		{
			TermType::Sum => result += &format!("({})", self.numerator.print()),
			_ => result += &self.numerator.print(),
		}
		result += " / ";
		match self.denominator.get_type()
		{
			TermType::Sum => result += &format!("({})", self.denominator.print()),
			_ => result += &self.denominator.print()
		}
		result
	}

	fn get_type(&self) -> TermType
	{
		TermType::Fraction
	}

	fn get_parts(&self) -> Vec<Box<dyn Term>> {
		vec![self.numerator.copy(), self.denominator.copy()]
	}

	fn copy(&self) -> Box<dyn Term> {
		Box::new(Fraction::new(self.numerator.copy(), self.denominator.copy()))
	}
}

impl Fraction
{
	pub fn new(numerator: Box<dyn Term>, denominator: Box<dyn Term>) -> Fraction
	{
		Fraction { numerator: numerator, denominator: denominator }
	}
}