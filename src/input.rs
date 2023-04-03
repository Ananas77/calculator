use crate::{term::*, sum::Sum, product::Product, variable::Variable, fraction::Fraction, power::Power};

pub fn term_from_string(input: &str) -> Result<Box<dyn Term>, String>
{
    let tokens = input.split_whitespace().collect::<Vec<_>>();
    let mut iter = tokens.iter().peekable();
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
            }
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
            Ok(Box::new(Product::new(vec![Box::new(Number::new(-1.0)), factor])))
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