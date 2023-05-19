use crate::{term::*, product::Product, sum::Sum, power::Power};

pub fn prime_factors(term: Box<dyn Term>) -> Product // assumes, the term is calculated already!!!
{
    let mut factors: Vec<Box<dyn Term>> = vec![];
    match term.get_type()
    {
        TermType::Product => {
            for factor in term.get_parts()
            {
                factors.extend(prime_factors(factor).get_parts());
            }
        },
        TermType::Sum => {
            // factor out the greatest common divisor
            factors.extend(factor_out_gcd(term).get_parts());
        },
        TermType::Number => {
            factors.extend(num_prime_factors(term.get_value() as i64));
        },
        TermType::Power => {
            factors.extend(prime_factors(term.get_parts()[0].copy()).get_parts().iter().map(|factor| Box::new(Power::new(factor.copy(), term.get_parts()[1].copy())) as Box<dyn Term>))
        }
        _ => factors.push(term)
    }
    Product::new(factors)
}

pub fn factor_out_gcd(term: Box<dyn Term>) -> Box<dyn Term> // works only for sums!!!
{
    let gcd = greatest_common_divisor(term.get_parts());
    let gcd_as_factors = prime_factors(gcd.copy()).get_parts();
    let mut factors = vec![];
    factors.extend(gcd_as_factors.iter().map(|factor| factor.copy()));
    factors.push(Sum::new(term.get_parts().iter().map(|summand| {
        let mut as_factors = prime_factors(summand.copy()).get_parts();
        for factor in &gcd_as_factors
        {
            as_factors = as_factors.into_iter()
                .enumerate()
                .filter(|(_, elem)| elem.print() != factor.print())
                .map(|(_, elem)| elem.copy())
                .collect();
        }
        Product::new(as_factors).calculate(false)
    }).collect())
        .calculate(false));
    Box::new(Product::new(factors))
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
				if factor == other_factor
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

pub fn greatest_common_divisor(input: Vec<Box<dyn Term>>) -> Box<dyn Term>
{
    let all_factors: Vec<Vec<Box<dyn Term>>> = input.iter().map(|x| prime_factors(x.copy()).get_parts()).collect(); // a vector with vectors with the prime factors of all inputs
    let mut all_indiv_factors = vec![];
    for vector in &all_factors
    {
        all_indiv_factors.extend(vector);
    }
    all_indiv_factors.sort_by(|a, b| a.print().cmp(&b.print()));
    all_indiv_factors.dedup_by(|a, b| a == b);
    let mut result = Vec::new();
    for &factor in &all_indiv_factors {
        let mut min_occurrences = std::usize::MAX;
        for v in &all_factors {
            let count = v.iter().filter(|&x| x == factor).count();
            if count < min_occurrences {
                min_occurrences = count;
            }
        }
        for _ in 0..min_occurrences {
            result.push(factor.copy());
        }
    }
    if result.len() == 0
    {
        result = vec![Box::new(Number::new(1.0))]
    }
    Product::new(result).calculate(false)
}

fn num_prime_factors(n: i64) -> Vec<Box<dyn Term>>
{
    let mut factors: Vec<Box<dyn Term>> = Vec::new();
    let mut remainder = n;

    for i in 2..(n.abs()+1)
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

    if remainder == -1
    {
        factors.push(Box::new(Number::new(-1.0)))
    }

    factors
}