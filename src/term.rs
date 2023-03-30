use std::fmt;

pub trait Term
{
    fn solve(&self) -> Box<dyn Term> {Box::new(Number{value:0.0})}
}

impl fmt::Display for dyn Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", 0.0)
    }
}

pub struct Number
{
    value:f32
}

impl Term for Number
{
    fn solve(&self) -> Box<dyn Term>
    {
        Box::new(Number{value:self.value})
    }
}

impl Number
{
    pub fn new(value:f32) -> Self
    {
        Number
        {
            value:value
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}