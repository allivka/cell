use std::fmt;
use crate::executor::ExecutorError::NotImplemented;
use derive_more::Debug;

#[derive(Debug, Clone)]
pub enum ExecutorError {
    #[debug("ExecutorError::NotImplemented -> {}", _0)]
    NotImplemented(String),

    #[debug("ExecutorError::NotFound -> {}", _0)]
    NotFound(String),

    #[debug("ExecutorError::IoFailure -> {}", _0)]
    IoFailure(String),

    #[debug("ExecutorError::Custom -> {}", _0)]
    Custom(String),
}

impl fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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