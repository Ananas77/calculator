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