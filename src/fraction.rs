use std::vec;

use crate::{term::*, math::prime_factors, product::Product};

pub struct Fraction
{
    numerator: Box<dyn Term>,
    denominator: Box<dyn Term>
}

impl Term for Fraction
{
    fn calculate(&self) -> Box<dyn Term> {
        let result: Box<dyn Term>;  // will later be returned
        let prime_factors_numerator = prime_factors(self.numerator.calculate()).get_parts();    // get numerator and denominator into products, reduce the fraction
        let prime_factors_denominator = prime_factors(self.denominator.calculate()).get_parts();
        let mut reduced_factors: Vec<Box<dyn Term>> = vec![];   // all factors the fraction should be reduced by get in here
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
            if reduced_by_factor
            {
                reduced_factors.push(factor_numerator.copy());
            }
            else {
                new_factors_numerator.push(factor_numerator.copy());
            }
        }

        // calculate the resulting fraction
        let new_numerator = Product::new(new_factors_numerator).calculate();
        result = match new_factors_denominator.len()
        {
            0 => new_numerator,    // check wether the result is a fraction
            _ =>
            {
                let new_denominator = Product::new(new_factors_denominator).calculate();
                Box::new(Fraction::new(new_numerator, new_denominator))
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
}

impl Fraction
{
    pub fn new(numerator: Box<dyn Term>, denominator: Box<dyn Term>) -> Fraction
    {
        Fraction { numerator: numerator, denominator: denominator }
    }
}