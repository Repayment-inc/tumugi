use std::error::Error;

pub trait TaskExecutor {
    fn execute(&self, input: &str) -> Result<String, Box<dyn Error>>;
}