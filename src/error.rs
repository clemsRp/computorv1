
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

    pub fn at_if_none(mut self, position: usize) -> Self {
        if self.position.is_none() {
            self.position = Some(position);
        }
        self
    }

    pub fn report(&self, input: &str) {
        eprintln!("Error: {}", self.message);
        if let Some(pos) = self.position {
            eprintln!("    {}", input);
            let indent: String = input
                .chars()
                .take(pos)
                .map(|c| if c == '\t' { '\t' } else { ' ' })
                .collect();
            eprintln!("    {}^", indent);
        }
    }
}
