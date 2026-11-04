use std::fmt;
use std::io::{Read, Write};

use derive_more::Debug;

#[derive(Debug, Clone)]
pub enum TaskError {

    #[debug("TaskError::IoFailure -> {}", _0)]
    IoFailure(String),

    #[debug("TaskError::ProcessFailure -> {}", _0)]
    ProcessFailure(String),

    #[debug("TaskError::ThreadFailure -> {}", _0)]
    ThreadFailure(String),

    #[debug("TaskError::TaskFinished -> {}", _0)]
    TaskFinished(String),

    #[debug("TaskError::Custom -> {}", _0)]
    Custom(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type TaskResult<T> = Result<T, TaskError>;

pub trait Task<T> {
    fn stdin(&mut self) -> Option<&mut dyn Write>;
    fn stdout(&mut self) -> Option<&mut dyn Read>;
    fn stderr(&mut self) -> Option<&mut dyn Read>;
    fn wait(&mut self) -> TaskResult<T>;

    fn is_finished(&mut self) -> bool;
}