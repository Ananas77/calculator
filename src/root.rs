use std::{collections::HashMap, vec};

use crate::{term::{Term, TermType, Number}, math::prime_factors, power::Power, sum::Sum, product::Product, fraction::Fraction};

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
        if calculated_index.get_type() == TermType::Error
		{
			return calculated_index
		}
		if calculated_radicand.get_type() == TermType::Error
		{
			return calculated_radicand
		}

        let prime_factors_radicand = prime_factors(calculated_radicand.copy()).get_parts();
        let mut factors_map: HashMap<Box<dyn Term>, Box<dyn Term>> = HashMap::new();
        let mut new_factors: Vec<Box<dyn Term>> = Vec::new();
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
            let exponent = Box::new(Fraction::new(power.1.copy(), calculated_index.copy())).calculate(round);
            match exponent.get_type()
            {
                TermType::Sum => new_factors.extend(exponent.get_parts().iter().map(|summand| Box::new(Power::new(power.0.copy(), summand.copy())) as Box<dyn Term>)),
                TermType::Fraction => {
                    if exponent.get_parts()[1].get_type() == TermType::Number
                    {
                        match exponent.get_parts()[0].copy().get_type()
                        {
                            TermType::Number => {
                                let new_exp = exponent.get_parts()[0].get_value() % exponent.get_parts()[1].get_value();
                                if new_exp != exponent.get_parts()[0].get_value()
                                {
                                    new_factors.extend(vec![Box::new(Power::new(power.0.copy(), Box::new(Number::new(f64::floor(exponent.calculate(true).get_value()))))) as Box<dyn Term>,
                                    Box::new(Power::new(power.0.copy(), Fraction::new(Box::new(Number::new(new_exp)), exponent.get_parts()[1].copy()).calculate(round)))]);
                                }
                                else {
                                    new_factors.push(Box::new(Root::new(exponent.get_parts()[1].copy(), Power::new(power.0.copy(), exponent.get_parts()[0].copy()).calculate(round))))
                                }
                            },
                            _ => new_factors.push(Box::new(Root::new(exponent.get_parts()[1].copy(), Power::new(power.0.copy(), exponent.get_parts()[0].copy()).calculate(round))))
                        }
                    }
                },
                _ => new_factors.push(Box::new(Power::new(power.0.copy(), exponent)).calculate(round))
            }
        }

        result = match new_factors.len()
        {
            0 => Box::new(Number::new(1.0)),
            1 => {
                new_factors[0].copy()
            },
            _ => Product::new(new_factors).calculate(round)
        };

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
            TermType::Number => result += &self.index.print(),
            _ => result += &format!("({})", self.index.print()),
        }
        result += "rt";
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