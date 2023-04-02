use crate::{term::*, sum::Sum, product::Product, variable::Variable, fraction::Fraction, power::Power};

pub fn term_from_string(input: String) -> Box<dyn Term>
{
    let mut result: Box<dyn Term> = Box::new(Number::new(0.0));
    let parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    let mut interpret_next_as: TermType = TermType::None;
    for part in parts
    {
        if let Ok(i) = part.parse::<f64>()
        {
            match interpret_next_as
            {
                TermType::Sum => result = Box::new(Sum::new(vec![result, Box::new(Number::new(i))])),
                TermType::Product => result = Box::new(Product::new(vec![result, Box::new(Number::new(i))])),
                TermType::Fraction => result = Box::new(Fraction::new(result, Box::new(Number::new(i)))),
                TermType::Power => result = Box::new(Power::new(result, Box::new(Number::new(i)))),
                _ => result = Box::new(Number::new(i))
            }
            interpret_next_as = TermType::None;
            continue;
        }
        if part == "+"
        {
            interpret_next_as = TermType::Sum;
            continue;
        }
        if part == "*"
        {
            interpret_next_as = TermType::Product;
            continue;
        }
        if part == "/"
        {
            interpret_next_as = TermType::Fraction;
            continue;
        }
        if part == "^"
        {
            interpret_next_as = TermType::Power;
            continue;
        }
        match interpret_next_as
        {
            TermType::Sum => result = Box::new(Sum::new(vec![result, Box::new(Variable::new(&part))])),
            TermType::Product => result = Box::new(Product::new(vec![result, Box::new(Variable::new(&part))])),
            TermType::Fraction => result = Box::new(Fraction::new(result, Box::new(Variable::new(&part)))),
            TermType::Power => result = Box::new(Power::new(result, Box::new(Variable::new(&part)))),
            _ => result = Box::new(Variable::new(&part))
        }
    }
    result
}