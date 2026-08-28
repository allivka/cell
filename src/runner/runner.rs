use crate::base::{format_custom};
use crate::executor::ExecutorError;
use crate::runner::RunnerError::NotImplemented;

#[derive(Debug, Clone)]
pub enum RunnerError {
    Custom(String),
    NotImplemented(String),
    InvalidInput(String),
    IoFailure(String),
    ProcessFailure(String),
    ExecutorFailure(ExecutorError)
}

impl std::fmt::Display for RunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format_custom("RunnerError", self.to_string()))
    }
}

pub type RunnerResult<T> = Result<T, RunnerError>;

pub trait Runner {

    fn name(&self) -> &str {
        "unimplemented runner"
    }

    fn run(&self, _input: String) -> RunnerResult<String> {
        Err(NotImplemented(format!("{} is not implemented", self.name())))
    }

    fn run_mut(&mut self, input: String) -> RunnerResult<String> {
        self.run(input)
    }
}

