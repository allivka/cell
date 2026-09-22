use crate::executor::ExecutorError;
use derive_more::Debug;
use crate::parser::ParserError;

#[derive(Debug, Clone)]
pub enum RunnerError {
    #[debug("RunnerError::Custom -> {}", _0)]
    Custom(String),
    
    #[debug("RunnerError::NotImplemented -> {}", _0)]
    NotImplemented(String),
    
    #[debug("RunnerError::InvalidInput -> {}", _0)]
    InvalidInput(String),
    
    #[debug("RunnerError::IoFailure -> {}", _0)]
    IoFailure(String),

    #[debug("RunnerError::ProcessFailure -> {}", _0)]
    ProcessFailure(String),

    #[debug("RunnerError::ExecutorFailure -> {}", _0)]
    ExecutorFailure(ExecutorError),

    #[debug("RunnerError::ParserFailure -> {}", _0)]
    ParserFailure(ParserError)
}

impl std::fmt::Display for RunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type RunnerResult<T> = Result<T, RunnerError>;

pub trait Runner {

    fn name(&self) -> &str;

    fn run(&self, _input: &String) -> RunnerResult<String>;

    // fn run_mut(&mut self, input: String) -> RunnerResult<String> {
    //     self.run(input)
    // }
}

