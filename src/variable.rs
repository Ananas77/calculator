use crate::term::*;

pub struct Variable
{
    name: String
}

impl Term for Variable
{
    fn calculate(&self) -> Box<dyn Term>
    {
        Box::new(Variable::new(&self.name))
    }

    fn print(&self) -> String
    {
        format!("{}", self.name)
    }

    fn get_type(&self) -> TermType {
		TermType::Variable
	}

    fn copy(&self) -> Box<dyn Term> {
        Box::new(Variable::new(&self.name))
    }
}

impl Variable
{
    pub fn new(name: &str) -> Variable
    {
        Variable
        {
            name: name.to_string()
        }
    }
}