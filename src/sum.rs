use crate::term::*;

pub struct Sum
{
    summands: Vec<Box<dyn Term>>
}

impl Term for Sum
{
    fn calculate(&self) -> Box<dyn Term> {
        let result:Box<dyn Term>;
        let mut number_result = Number::new(0.0);
        for term in &self.summands
        {
            let calculated_term = term.calculate();
            if calculated_term.get_type() == TermType::Number
            {
                number_result = Number::new(number_result.get_value() + calculated_term.get_value());
            };
        };
        result = Box::new(number_result);
        let immut_result = result;
        immut_result
    }

    fn get_type(&self) -> TermType {
        TermType::Sum
    }
}

impl Sum
{
    pub fn new(summands:Vec<Box<dyn Term>>) -> Sum
    {
        Sum
        {
            summands:summands
        }
    }
}