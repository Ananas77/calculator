use crate::term::{Term, TermType};

pub struct Power
{
    base: Box<dyn Term>,
    exponent: Box<dyn Term>
}

impl Term for Power
{
    fn print(&self) -> String {
        let mut result = String::new();
        match self.base.get_type()
        {
            TermType::Number => result += &self.base.print(),
            _ => result += &format!("({})", self.base.print()),
        }
        result += " ^ ";
        match self.exponent.get_type()
        {
            TermType::Power => result += &self.exponent.print(),
            TermType::Number => result += &self.exponent.print(),
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