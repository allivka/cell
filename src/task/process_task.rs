use std::process::{Child, Command};
use crate::task::*;
use std::io::{Read, Write};

pub struct ProcessTask {
    child: Child,
    connector: TaskPipeConnector
}

impl ProcessTask {
    pub fn spawn(command: &mut Command) -> TaskResult<ProcessTask> {

        let pipes = match StreamPipes::new() {
            Ok(pipes) => pipes,
            Err(e) => return Err(TaskError::Custom(e))
        };

        Ok(ProcessTask {
            child: match command.stdin(pipes.stdin.0).stdout(pipes.stdout.1).stderr(pipes.stderr.1).spawn() {
                Ok(v) => v,
                Err(e) => return match e.kind() {
                    std::io::ErrorKind::NotFound => Err(TaskError::NotFound(String::from("command not found"))),
                    _ => Err(TaskError::CommandFailure(e.to_string()))
                },
            },

            connector: TaskPipeConnector {
                stdin_writer: Some(Box::new(pipes.stdin.1)),
                stdout_reader: Some(Box::new(pipes.stdout.0)),
                stderr_reader: Some(Box::new(pipes.stderr.0)),
            }
        })
    }

}

impl Task<TaskResult<String>> for ProcessTask {

    fn stdin(&mut self) -> Option<&mut dyn Write> {
        self.connector.stdin_writer.as_mut().map(|s| s as &mut dyn Write)
    }

    fn stdout(&mut self) -> Option<&mut dyn Read> {
        self.connector.stdout_reader.as_mut().map(|s| s as &mut dyn Read)
    }

    fn stderr(&mut self) -> Option<&mut dyn Read> {
        self.connector.stderr_reader.as_mut().map(|s| s as &mut dyn Read)
    }

    fn wait(&mut self) -> TaskResult<TaskResult<String>> {
        match self.child.wait() {
            Ok(status) => match status.success() {
                true => Ok(Ok(status.to_string())),
                false => Ok(Err(TaskError::ProcessFailure(status.to_string())))
            },
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