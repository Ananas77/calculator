use std::{vec};

use crate::{term::*, math::prime_factors, product::Product};

pub struct Fraction
{
	numerator: Box<dyn Term>,
	denominator: Box<dyn Term>
}

impl Term for Fraction
{
	fn calculate(&self, rounded: bool) -> Box<dyn Term> {
		let result: Box<dyn Term>;  // will later be returned
		// calculate numerator and denominator
		let mut calculated_numerator = self.numerator.calculate(rounded);
		let mut calculated_denominator = self.denominator.calculate(rounded);
		match calculated_numerator.get_type()
		{
			TermType::Fraction => {
				calculated_denominator = Product::new(vec![calculated_numerator.get_parts()[1].copy(), calculated_denominator]).calculate(rounded);
				calculated_numerator = calculated_numerator.get_parts()[0].copy()
			}
			_ => {}
		}
		match calculated_denominator.get_type()
		{
			TermType::Fraction => {
				calculated_numerator = Product::new(vec![calculated_denominator.get_parts()[1].copy(), calculated_numerator]).calculate(rounded);
				calculated_denominator = calculated_denominator.get_parts()[0].copy()
			}
			_ => {}
		}
		// reduce the fraction
		let prime_factors_numerator = prime_factors(calculated_numerator).get_parts();    // get numerator and denominator into products, reduce the fraction
		let prime_factors_denominator = prime_factors(calculated_denominator).get_parts();
		let mut new_factors_numerator: Vec<Box<dyn Term>> = vec![]; // what is left from the numerator after reducing
		let mut new_factors_denominator: Vec<Box<dyn Term>> = vec![];   // what is left from the denominator after reducing
		// go through each prime factor in the numerator and check wether the fraction can be reduced by it
		for factor in &prime_factors_denominator
		{
			new_factors_denominator.push(factor.copy());
		}
		for factor_numerator in &prime_factors_numerator
		{
			let mut reduced_by_factor = false;
			let mut i = 0;
			for factor_denominator in &new_factors_denominator
			{
				if factor_numerator.print() == factor_denominator.print()
				{
					reduced_by_factor = true;
					new_factors_denominator.remove(i);
					break;
				}
				i += 1;
			}
			if !reduced_by_factor {
				new_factors_numerator.push(factor_numerator.copy());
			}
		}

		// calculate the resulting fraction
		let new_numerator = Product::new(new_factors_numerator).calculate(rounded);
		result = match new_factors_denominator.len()
		{
			0 => new_numerator,    // check wether the result is a fraction
			_ =>
			{
				let new_denominator = Product::new(new_factors_denominator).calculate(rounded);
				if rounded
				{
					let numerator_factors = prime_factors(new_numerator.copy()).get_parts();
					let denominator_factors = prime_factors(new_denominator.copy()).get_parts();
					let mut new_numerator_factors = vec![];
					let mut new_denominator_factors = vec![];
					let mut numerator_number = 1.0;
					let mut denominator_number = 1.0;
					// get all numbers from the numerator and denominator
					for factor in &numerator_factors
					{
						match factor.get_type()
						{
							TermType::Number => {
								numerator_number *= factor.get_value();
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
								denominator_number *= factor.get_value();
							},
							_ => {
								new_denominator_factors.push(factor.copy());
							}
						}
					};
					Product::new(vec![Box::new(Number::new(numerator_number / denominator_number)), Box::new(Fraction::new(Box::new(Product::new(new_numerator_factors)), Box::new(Product::new(new_denominator_factors))))]).calculate(rounded)
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