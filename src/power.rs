use std::vec;

use crate::{term::{Term, TermType, Number}, product::Product, math::factor_out_gcd, fraction::Fraction};

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
        
        result = match calculated_base.get_type() {
            TermType::Number => {
                match calculated_exponent.get_type() {
                    TermType::Number => Number::new(calculated_base.get_value().powf(calculated_exponent.get_value())).calculate(round),
                    TermType::Sum =>  {
                        let mut factors = vec![];
                        for summand in &calculated_exponent.get_parts()
                        {
                            match summand.get_type() {
                                TermType::Number => factors.push(Number::new(calculated_base.get_value().powf(summand.get_value())).calculate(round)),
                                _ => factors.push(Power::new(calculated_base.copy(), summand.copy()).calculate(round))
                            };
                        }
                        match factors.len()
                        {
                            1 => factors[0].copy(),
                            _ => Product::new(factors).calculate(round)
                        }
                    },
                    TermType::Product => {
                        let mut new_factors = vec![];
                        for factor in calculated_exponent.get_parts()
                        {
                            if factor.get_type() == TermType::Number
                            {
                                calculated_base = Number::new(calculated_base.get_value().powf(factor.get_value())).calculate(round);
                            }
                            else
                            {
                                new_factors.push(factor);
                            }
                        };
                        Box::new(Power::new(calculated_base, Product::new(new_factors).calculate(round)))
                    },
                    TermType::Fraction => todo!(),  // root
                    _ => Box::new(Power::new(calculated_base, calculated_exponent))
                }
            },
            TermType::Sum => {
                let mut sums = vec![];
                let mut not_sums = vec![];
                for factor in factor_out_gcd(calculated_base).get_parts()
                {
                    if factor.get_type() != TermType::Sum
                    {
                        not_sums.push(factor.copy());
                    }
                    else {
                        sums.push(factor.copy());
                    }
                };
                let mut factors = vec![];
                factors.extend(sums.iter().map(|sum| Box::new(Power::new(sum.copy(), calculated_exponent.copy())) as Box<dyn Term>));   // not calculated to prevent infinite recursion and stack overflow
                factors.extend(not_sums.iter().map(|sum| Box::new(Power::new(sum.copy(), calculated_exponent.copy())).calculate(round)));
                Box::new(Product::new(factors))
            },
            TermType::Product => {
                let mut factors = vec![];
                for factor in calculated_base.get_parts()
                {
                    factors.push(Box::new(Power::new(factor.copy(), calculated_exponent.copy())) as Box<dyn Term>);
                };
                Product::new(factors).calculate(round)
            },
            TermType::Variable => {
                match calculated_exponent.get_type() {
                    TermType::Sum =>  {
                        let mut factors = vec![];
                        for summand in &calculated_exponent.get_parts()
                        {
                            factors.push(Power::new(calculated_base.copy(), summand.copy()).calculate(round))
                        }
                        match factors.len()
                        {
                            1 => factors[0].copy(),
                            _ => Product::new(factors).calculate(round)
                        }
                    },
                    TermType::Product => {
                        let mut new_factors = vec![];
                        for factor in calculated_exponent.get_parts()
                        {
                            new_factors.push(factor);
                        };
                        Box::new(Power::new(calculated_base, Product::new(new_factors).calculate(round)))
                    },
                    TermType::Fraction => todo!(),  // root
                    _ => Box::new(Power::new(calculated_base, calculated_exponent))
                }
            },
            TermType::Fraction => {
                Fraction::new(Box::new(Power::new(calculated_base.get_parts()[0].copy(), calculated_exponent.copy())), Box::new(Power::new(calculated_base.get_parts()[1].copy(), calculated_exponent.copy()))).calculate(round)
            },
            _ => Box::new(Power::new(calculated_base, calculated_exponent))
        };
        result
    }

    fn print(&self) -> String {
        let mut result = String::new();
        match self.base.get_type()
        {
            TermType::Number => result += &self.base.print(),
            TermType::Variable => result += &self.base.print(),
            _ => result += &format!("({})", self.base.print()),
        }
        result += " ^ ";
        match self.exponent.get_type()
        {
            TermType::Power => result += &self.exponent.print(),
            TermType::Number => result += &self.exponent.print(),
            TermType::Variable => result += &self.exponent.print(),
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