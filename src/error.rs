
pub struct CalcError {
    message: String,
    position: Option<usize>,
}

pub type CalcResult<T> = Result<T, CalcError>;

impl CalcError {
    pub fn new(message: impl Into<String>) -> Self {
        CalcError { message: message.into(), position: None }
    }
 
    pub fn at(position: usize, message: impl Into<String>) -> Self {
        CalcError { message: message.into(), position: Some(position) }
    }

    pub fn report(&self, input: String) {
        eprintln!("Error: {}", self.message);
        if let Some(pos) = self.position {
            eprintln!("    {}", input);
            eprintln!("    {}^", " ".repeat(pos));
        }
    }
}
