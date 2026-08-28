use std::fmt;
use crate::base::format_custom;
use crate::executor::ExecutorError::NotImplemented;

#[derive(Debug, Clone)]
pub enum ExecutorError {
    NotImplemented(String),
    NotFound(String),
    IoFailure(String),
    Custom(String),
}

impl fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format_custom("ExecutorError", self.to_string()))
    }
}

pub type ExecutorResult<T> = Result<T, ExecutorError>;

pub type ExecutorArgs = Vec<String>;

pub trait Executor {

    fn name(&self) -> &str {
        "Unnamed executor"
    }

    fn execute(&self, _args: &ExecutorArgs) -> ExecutorResult<String> {
        Err(NotImplemented(format!("{} is not implemented(immutable)", self.name())))
    }

    fn execute_mut(&self, _args: &ExecutorArgs) -> ExecutorResult<String> {
        self.execute(_args)
    }
}

pub struct UnnamedExecutor {}
impl Executor for UnnamedExecutor {}