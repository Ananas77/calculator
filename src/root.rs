use std::{collections::HashMap, vec};

use crate::{term::{Term, TermType, Number}, math::prime_factors, power::Power, sum::Sum, product::Product};

pub struct Root
{
    index: Box<dyn Term>,
    radicand: Box<dyn Term>
}

impl Term for Root
{
    fn calculate(&self, round: bool) -> Box<dyn Term> {
        let result:Box<dyn Term>;
        let calculated_index = self.index.calculate(round);
        let calculated_radicand = self.radicand.calculate(round);
        let mut coefficient: Box<dyn Term> = Box::new(Number::new(1.0));

        let prime_factors_radicand = prime_factors(calculated_radicand.copy()).get_parts();
        let mut factors_map: HashMap<Box<dyn Term>, Box<dyn Term>> = HashMap::new();
        for factor in &prime_factors_radicand
        {
            let factor_as_power = if factor.get_type() != TermType::Power
            {
                Box::new(Power::new(factor.copy(), Box::new(Number::new(1.0))))
            }
            else 
            {
                factor.copy()
            };
            if let Some(&ref val) = factors_map.get(&factor_as_power.get_parts()[0])
            {
                factors_map.insert(factor_as_power.get_parts()[0].copy(), Sum::new(vec![val.copy(), factor_as_power.get_parts()[1].copy()]).calculate(round));
            }
            else {
                factors_map.insert(factor_as_power.get_parts()[0].copy(), factor_as_power.get_parts()[1].copy());
            }
        }
        for power in factors_map.iter().map(|f| (f.0.copy(), f.1.copy())).collect::<Vec<(Box<dyn Term>, Box<dyn Term>)>>()
        {
            if power.1.get_type() == TermType::Number && calculated_index.get_type() == TermType::Number
            {
                let mut exponent_value = power.1.get_value();
                while exponent_value >= calculated_index.get_value()
                {
                    coefficient = Product::new(vec![coefficient, power.0.copy()]).calculate(round);
                    exponent_value -= calculated_index.get_value();
                }
                factors_map.insert(power.0.copy(), Box::new(Number::new(exponent_value)));
            }
        }

        let new_radicand = Product::new(factors_map.iter().map(|factor| Box::new(Power::new(factor.0.copy(), factor.1.copy())) as Box<dyn Term>).collect()).calculate(round);

        if new_radicand == Box::new(Number::new(1.0)) {
            return coefficient
        }
        if coefficient != Box::new(Number::new(1.0))
        {
            result = Box::new(Product::new(vec![coefficient, Box::new(Root::new(calculated_index, new_radicand))]))
        }
        else {
            result = Box::new(Root::new(calculated_index, new_radicand))
        }

        result
    }

    fn get_type(&self) -> TermType {
        TermType::Root
    }

    fn get_parts(&self) -> Vec<Box<dyn Term>> {
        vec![self.index.copy(), self.radicand.copy()]
    }

    fn copy(&self) -> Box<dyn Term> {
        Box::new(Root::new(self.index.copy(), self.radicand.copy()))
    }

    fn print(&self) -> String {
        let mut result = String::new();
        match self.index.get_type()
        {
            TermType::Number | TermType::Variable => result += &self.index.print(),
            _ => result += &format!("({})", self.index.print()),
        }
        result += "rt ";
        result += &format!("({})", self.radicand.print());
        result
    }
}

impl Root
{
    pub fn new(index: Box<dyn Term>, radicand: Box<dyn Term>) -> Root
    {
        Root { index: index, radicand: radicand }
    }
}