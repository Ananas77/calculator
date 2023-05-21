use std::{vec, collections::HashMap};

use crate::{term::*, math::prime_factors, product::Product, sum::Sum, power::Power};

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
		if calculated_numerator.get_type() == TermType::Error
		{
			return calculated_numerator
		}
		if calculated_denominator.get_type() == TermType::Error
		{
			return calculated_denominator
		}
		if calculated_denominator == Box::new(Number::new(0.0))
		{
			return Box::new(Error::new("Can't divide by zero!".to_string()))
		}
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

		// check for sums
		if calculated_numerator.get_type() == TermType::Sum
		{
			return Box::new(Sum::new(calculated_numerator.get_parts().iter().map(|summand|  Fraction::new(summand.copy(), calculated_denominator.copy()).calculate(round)).collect()))
		}

		// reduce the fraction
		// this code expresses the numerators and denominators prime factors as powers, stores the numerator and denominator in a hashmap where the key is the base and the exponent is the value (the denominator's exponents are negated)
		let prime_factors_numerator = prime_factors(calculated_numerator).get_parts();
		let prime_factors_denominator = prime_factors(calculated_denominator).get_parts();
		let mut new_factors: HashMap<Box<dyn Term>, Box<dyn Term>> = HashMap::new();
		for factor in &prime_factors_numerator
		{
			if factor.get_type() == TermType::Number && factor.get_value() == -1.0 // this will be stored as a coefficient later
			{
				fraction_is_neg *= -1.0;
			}
			else {
				// expresses factor as power, stores it in the hashmap
				let factor_as_power = if factor.get_type() != TermType::Power
				{
					Box::new(Power::new(factor.copy(), Box::new(Number::new(1.0))))
				}
				else 
				{
					factor.copy()
				};
				if let Some(&ref val) = new_factors.get(&factor_as_power.get_parts()[0])
				{
					new_factors.insert(factor_as_power.get_parts()[0].copy(), Sum::new(vec![val.copy(), factor_as_power.get_parts()[1].copy()]).calculate(round));
				}
				else {
					new_factors.insert(factor_as_power.get_parts()[0].copy(), factor_as_power.get_parts()[1].copy());
				}
			}
		}
		for factor in &prime_factors_denominator
		{
			if factor.get_type() == TermType::Number && factor.get_value() == -1.0
			{
				fraction_is_neg *= -1.0;
			}
			else {
				let factor_as_power = if factor.get_type() != TermType::Power
				{
					Box::new(Power::new(factor.copy(), Box::new(Number::new(1.0))))
				}
				else 
				{
					factor.copy()
				};
				if let Some(&ref val) = new_factors.get(&factor_as_power.get_parts()[0])
				{
					new_factors.insert(factor_as_power.get_parts()[0].copy(), Sum::new(vec![val.copy(), Box::new(Product::new(vec![factor_as_power.get_parts()[1].copy(), Box::new(Number::new(-1.0))]))]).calculate(round));
				}
				else {
					new_factors.insert(factor_as_power.get_parts()[0].copy(), Box::new(Product::new(vec![factor_as_power.get_parts()[1].copy(), Box::new(Number::new(-1.0))])).calculate(round));
				}
			}
		}

		// calculate the resulting fraction
		// the hashmap is converted to a vector, powers with positive exponents are stored in the numerator, those with negative exponents are stored in the denominator
		let new_factors_as_vec: Vec<(Box<dyn Term>, Box<dyn Term>)> = new_factors.into_iter().collect();
		let new_numerator = Product::new(new_factors_as_vec.iter()
			.filter(|(_, exp)| !prime_factors(exp.copy()).get_parts().contains(&(Box::new(Number::new(-1.0)) as Box<dyn Term>)))
			.map(|(base, exp)| Box::new(Power::new(base.copy(), exp.copy())) as Box<dyn Term>).collect()).calculate(round);
		let new_denominator = Product::new(new_factors_as_vec.iter()
			.filter(|(_, exp)| prime_factors(exp.copy()).get_parts().contains(&(Box::new(Number::new(-1.0)) as Box<dyn Term>)))
			.map(|(base, exp)| Box::new(Power::new(base.copy(), Box::new(Product::new(vec![exp.copy(), Box::new(Number::new(-1.0))])))) as Box<dyn Term>).collect()).calculate(round);
		result = if new_denominator == Box::new(Number::new(1.0)) as Box<dyn Term>
		{
			new_numerator    // check wether the result is a fraction
		}
		else
		{
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
		};

		// store -1 as a fraction coefficient, if the fraction is negative, return
		if fraction_is_neg == 1.0
		{
			result
		}
		else
		{
			Box::new(Product::new(vec![Box::new(Number::new(-1.0)), result]))
		}
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