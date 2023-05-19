use std::{vec};

use crate::{term::{Term, TermType, Number}, product::Product, math::prime_factors, fraction::Fraction};

pub struct Power
{
    base: Box<dyn Term>,
    exponent: Box<dyn Term>
}

impl Term for Power
{
    fn calculate(&self, round: bool) -> Box<dyn Term> {
        let result: Box<dyn Term>;
        let mut calculated_base = self.base.calculate(round);
        let mut calculated_exponent = self.exponent.calculate(round);
        match calculated_base.get_type() {
            TermType::Power => {
                calculated_exponent = Product::new(vec![calculated_exponent.copy(), calculated_base.get_parts()[1].copy()]).calculate(round);
                calculated_base = calculated_base.get_parts()[0].copy();
            }
            _ => {}
        }
        if prime_factors(calculated_exponent.copy()).get_parts().contains(&(Box::new(Number::new(-1.0)) as Box<dyn Term>))
        {
            return Box::new(Fraction::new(Box::new(Number::new(1.0)), Power::new(calculated_base, Box::new(Product::new(vec![calculated_exponent, Box::new(Number::new(-1.0))]))).calculate(round)))
        }
        
        result = match calculated_exponent.get_type() {
            TermType::Number => {
                if calculated_exponent.get_value() != 0.0
                {
                    match calculated_base.get_type() {
                        TermType::Number => Number::new(calculated_base.get_value().powf(calculated_exponent.get_value())).calculate(round),
                        TermType::Sum | TermType::Product | TermType::Fraction => if calculated_exponent.get_value().floor() == calculated_exponent.get_value()
                        {
                            Product::new(vec![calculated_base; (calculated_exponent.get_value() as i64).try_into().unwrap()]).calculate(round)
                        }
                        else
                        {
                            Box::new(Power::new(calculated_base, calculated_exponent))
                        },
                        _ => if calculated_exponent.get_value() != 1.0
                        {
                            Box::new(Power::new(calculated_base, calculated_exponent))
                        }
                        else {
                            calculated_base
                        }
                    }
                }
                else if calculated_base.get_type() == TermType::Number && calculated_base.get_value() == 0.0
                {
                    panic!("Can't calculate 0 to the power of 0!")
                }
                else
                {
                    Box::new(Number::new(1.0))
                }
            },
            TermType::Fraction => {
                todo!() // root
            }
            _ => Box::new(Power::new(calculated_base, calculated_exponent))
        };
        result
    }

    fn print(&self) -> String {
        let mut result = String::new();
        match self.base.get_type()
        {
            TermType::Number | TermType::Variable => result += &self.base.print(),
            _ => result += &format!("({})", self.base.print()),
        }
        result += "^";
        match self.exponent.get_type()
        {
            TermType::Power | TermType::Number | TermType::Variable => result += &self.exponent.print(),
            _ => result += &format!("({})", self.exponent.print()),
        }
        result
    }

    fn get_type(&self) -> TermType {
        TermType::Power
    }

    fn copy(&self) -> Box<dyn Term> {
        Box::new(Power::new(self.base.copy(), self.exponent.copy()))
    }

    fn get_parts(&self) -> Vec<Box<dyn Term>> {
        vec![self.base.copy(), self.exponent.copy()]
    }
}

impl Power
{
    pub fn new(base: Box<dyn Term>, exponent: Box<dyn Term>) -> Power
    {
        Power { base: base, exponent: exponent }
    }
}