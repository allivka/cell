use crate::executor::*;
use crate::executor::ExecutorError::{NotFound, IoFailure};

pub struct ExecutorPWD {}
impl Executor for ExecutorPWD {
    fn name(&self) -> &str {
        "pwd"
    }

    fn execute(&self, _args: &ExecutorArgs) -> ExecutorResult<String> {
        match std::env::current_dir() {
            Ok(dir) => match dir.to_str() {
                Some(dir) => Ok(dir.to_string()),
                None => Err(NotFound(String::from("Error: current working directory not found")))
            }
            Err(e) => Err(IoFailure(format!("{} error; could not read current working directory: {}", self.name(), e)))
        }
    }
}