use crate::{term::*, sum::Sum, product::Product, variable::Variable, fraction::Fraction, power::Power, root::Root};

pub fn term_from_string(input: &str) -> Result<Box<dyn Term>, String>
{
    let chars = input.replace(" ", "").replace("\n", "").replace("\t", "").chars().collect::<Vec<_>>();
    let mut tokens: Vec<String> = vec![];
    let mut current_input = "".to_string();
    for char in chars
    {
        if char == '+' || char == '-' || char == '*' || char == '/' || char == '^' || char == '(' || char == ')'
        {
            if current_input != ""
            {
                tokens.push(current_input.as_str().to_string());
                current_input = "".to_string();
            }
            tokens.push(char.to_string());
        }
        else if char == '.' || char == ',' {
            current_input += &'.'.to_string();
        }
        else
        {
            current_input += &char.to_string();
        }
    }
    if current_input != ""
    {
        tokens.push(current_input.as_str().to_string());
    }
    let new_tokens: Vec<&str> = tokens.iter().map(|token| token.as_str()).collect();
    let mut iter = new_tokens.iter().peekable();
    let term = parse_expr(&mut iter);
    term
}

fn parse_expr(iter: &mut std::iter::Peekable<std::slice::Iter<&str>>) -> Result<Box<dyn Term>, String>
{
    let mut term = parse_term(iter)?;
    while let Some(&op) = iter.peek() {
        match op {
            &"+" => {
                iter.next();
                let rhs = parse_term(iter)?;
                term = Box::new(Sum::new(vec![term, rhs]));
            }
            &"-" => {
                iter.next();
                let rhs = parse_term(iter)?;
                term = Box::new(Sum::new(vec![term, Box::new(Product::new(vec![Box::new(Number::new(-1.0)), rhs]))]));
            }
            _ => break,
        }
    }
    Ok(term)
}

fn parse_term(iter: &mut std::iter::Peekable<std::slice::Iter<&str>>) -> Result<Box<dyn Term>, String> {
    let mut term = parse_factor(iter)?;
    while let Some(&op) = iter.peek() {
        match op {
            &"*" => {
                iter.next();
                let rhs = parse_factor(iter)?;
                term = Box::new(Product::new(vec![term, rhs]));
            }
            &"/" => {
                iter.next();
                let rhs = parse_factor(iter)?;
                term = Box::new(Fraction::new(term, rhs));
            }
            _ => break,
        }
    }
    Ok(term)
}

fn parse_factor(iter: &mut std::iter::Peekable<std::slice::Iter<&str>>) -> Result<Box<dyn Term>, String> {
    let mut term = parse_power_part(iter)?;
    while let Some(&op) = iter.peek() {
        match op {
            &"^" => {
                iter.next();
                let rhs = parse_power_part(iter)?;
                term = Box::new(Power::new(term, rhs));
            },
            &"rt" => {
                iter.next();
                let rhs = parse_power_part(iter)?;
                term = Box::new(Root::new(term, rhs));
            },
            _ => break,
        }
    }
    Ok(term)
}

fn parse_power_part(iter: &mut std::iter::Peekable<std::slice::Iter<&str>>) -> Result<Box<dyn Term>, String> {
    match iter.next().ok_or_else(|| "Unexpected end of input".to_string())? {
        &"(" => {
            let term = parse_expr(iter)?;
            if iter.next() != Some(&")") {
                return Err("Expected ')'".to_string());
            }
            Ok(term)
        }
        &"-" => {
            let factor = parse_power_part(iter)?;
            Ok(Box::new(Product::new(vec![Box::new(Number::new(-1.0)), factor])).calculate(false))
        }
        &"sqrt" => {
            let radicand = parse_power_part(iter)?;
            Ok(Box::new(Root::new(Box::new(Number::new(2.0)), radicand)))
        }
        token => {
            if let Ok(val) = token.parse() {
                Ok(Box::new(Number::new(val)))
            } else {
                Ok(Box::new(Variable::new(token)))
            }
        }
    }
}