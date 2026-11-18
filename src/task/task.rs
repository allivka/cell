use std::fmt;
use std::io::{Read, Write};

use derive_more::Debug;
use crate::executor::ExecutorError;

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

    #[debug("TaskError::ExecutorFailure -> {}", _0)]
    ExecutorFailure(ExecutorError),

    #[debug("TaskError::CommandFailure -> {}", _0)]
    CommandFailure(String),

    #[debug("TaskError::NotFound -> {}", _0)]
    NotFound(String),

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

// Owned by task, reads from stdin and writes to stderr, stdout
pub struct TaskPipeOwner {
    pub stdin_reader: Option<Box<dyn Read>>,
    pub stdout_writer: Option<Box<dyn Write>>,
    pub stderr_writer: Option<Box<dyn Write>>,
}

// Owned from outside the task and connects to it, writes to stdin and reads stdout, stderr
pub struct TaskPipeConnector {
    pub stdin_writer: Option<Box<dyn Write>>,
    pub stdout_reader: Option<Box<dyn Read>>,
    pub stderr_reader: Option<Box<dyn Read>>,
}