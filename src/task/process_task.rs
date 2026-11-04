use std::process::{Child, ChildStdout, ExitCode, ExitStatus};
use crate::task::*;
use std::io::{Read, Write};

pub struct ProcessTask {
    child: Child,
}

impl ProcessTask {
    pub fn from(child: Child) -> Self {

        ProcessTask {
            child
        }
    }

}

impl Task<ExitStatus> for ProcessTask {

    fn stdin(&mut self) -> Option<&mut dyn Write> {
        self.child.stdin.as_mut().map(|s| s as &mut dyn Write)
    }

    fn stdout(&mut self) -> Option<&mut dyn Read> {
        self.child.stdout.as_mut().map(|s| s as &mut dyn Read)
    }

    fn stderr(&mut self) -> Option<&mut dyn Read> {
        self.child.stderr.as_mut().map(|s| s as &mut dyn Read)
    }

    fn wait(&mut self) -> TaskResult<ExitStatus> {
        match self.child.wait() {
            Ok(status) => Ok(status),
            Err(e) => Err(TaskError::ProcessFailure(e.to_string()))
        }
    }

    fn is_finished(&mut self) -> bool {
        match self.child.try_wait() {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(_) => true,
        }
    }

}