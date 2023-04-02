use crate::{term::*, product::Product};

pub fn prime_factors(term: Box<dyn Term>) -> Product // assumes, the term is solved already!!!
{
    let mut factors: Vec<Box<dyn Term>> = vec![];
    match term.get_type()
    {
        TermType::Product => {
            for factor in term.get_parts()
            {
                match factor.get_type()
                {
                    TermType::Number => {
                        factors.extend(num_prime_factors(factor.get_value() as i64));
                    },
                    _ => factors.push(factor),
                }
            }
        },
        TermType::Number => {
            factors.extend(num_prime_factors(term.get_value() as i64));
        },
        _ => factors.push(term)
    }
    Product::new(factors)
}

pub fn least_common_multiple(input: Vec<Box<dyn Term>>) -> Box<dyn Term>
{
    let mut factors: Vec<Box<dyn Term>> = vec![];
    for term in &input
	{
		let term_prime_factors = prime_factors(term.copy()).get_parts();
		let mut factors_copy: Vec<Box<dyn Term>> = factors.iter().map(|factor| factor.copy()).collect();
		for factor in &term_prime_factors
		{
			let mut i = 0;
			let mut already_in_vec = false;
			for other_factor in &factors_copy
			{
				if factor.print() == other_factor.print()
				{
					factors_copy.remove(i);
					already_in_vec = true;
					break;
				}
				i += 1;
			}
			if !already_in_vec
			{
				factors.push(factor.copy());
			}
		}
	}

    Product::new(factors).calculate(false)
}

fn num_prime_factors(n: i64) -> Vec<Box<dyn Term>>
{
    let mut factors: Vec<Box<dyn Term>> = Vec::new();
    let mut remainder = n;

    for i in 2..(n+1)
    {
        while remainder % i == 0
        {
            factors.push(Box::new(Number::new(i as f64)));
            remainder /= i;
        }
        if remainder == 1
        {
            break;
        }
    }

    factors
}