use os_pipe;
use std::io::{Read, Write};
use std::thread;
use crate::task::*;
use crate::core::*;

pub struct StreamPipes {
    pub stdin: (os_pipe::PipeReader, os_pipe::PipeWriter),
    pub stdout: (os_pipe::PipeReader, os_pipe::PipeWriter),
    pub stderr: (os_pipe::PipeReader, os_pipe::PipeWriter),
}

impl StreamPipes {
    pub fn new() -> Result<StreamPipes, String> {
        
        Ok(StreamPipes {
            stdin: match os_pipe::pipe() {
                Ok(v) => v,
                Err(e) => return Err(format!("{}: {e}", sformat_error("Failed creating pipe for stdin")))
            },

            stdout: match os_pipe::pipe() {
                Ok(v) => v,
                Err(e) => return Err(format!("{}: {e}", sformat_error("Failed creating pipe for stdout")))
            },

            stderr: match os_pipe::pipe() {
                Ok(v) => v,
                Err(e) => return Err(format!("{}: {e}", sformat_error("Failed creating pipe for stderr")))
            },
        })
    }
}

pub struct ThreadTask<T: Send + 'static> {
    handle: Option<thread::JoinHandle<T>>,
    connector: TaskPipeConnector

}

impl<T: Send + 'static> ThreadTask<T> {
    pub fn spawn<F>(f: F) -> TaskResult<ThreadTask<T>>
    where
        F: FnOnce(TaskPipeOwner) -> T, //stdin reader, stdout writer adn stderr writer
        F: Send + 'static {

        let  pipes = match StreamPipes::new() {
            Ok(v) => v,
            Err(e) => return Err(TaskError::ThreadFailure(e.to_string()))
        };

        Ok(Self {
            connector: TaskPipeConnector {
                stdin_writer: Some(Box::new(pipes.stdin.1)),
                stdout_reader: Some(Box::new(pipes.stdout.0)),
                stderr_reader: Some(Box::new(pipes.stderr.0)),
            },
            handle: Some(thread::spawn(move || {
                f(
                    TaskPipeOwner {
                        stdin_reader: Some(Box::new(pipes.stdin.0)),
                        stdout_writer: Some(Box::new(pipes.stdout.1)),
                        stderr_writer: Some(Box::new(pipes.stderr.1)),
                    }
                )
            })),

        })
    }
}

impl<T: Send + 'static> Task<T> for ThreadTask<T> {
    fn stdin(&mut self) -> Option<&mut dyn Write> {
        match self.connector.stdin_writer {
            Some(ref mut stdin) => Some(stdin.as_mut()),
            None => None
        }
    }
    
    fn stdout(&mut self) -> Option<&mut dyn Read> {
        match self.connector.stdout_reader {
            Some(ref mut stdout) => Some(stdout.as_mut()),
            None => None
        }
    }

    fn stderr(&mut self) -> Option<&mut dyn Read> {
        match self.connector.stderr_reader {
            Some(ref mut stderr) => Some(stderr.as_mut()),
            None => None
        }
    }

    fn wait(&mut self) -> TaskResult<T> {

        if let None = self.handle {
            return Err(TaskError::TaskFinished(sformat_info("task already finished")));
        }

        let handle = self.handle.take().expect(sformat_error("Failed waiting for thread task as it does not exist").as_str());

        let res = match handle.join() {
            Ok(v) => Ok(v),
            Err(_) => Err(TaskError::ThreadFailure("thread failed".to_string()))
        };

        res
    }

    fn is_finished(&mut self) -> bool {
        match self.handle {
            Some(ref handle) => handle.is_finished(),
            None => true,
        }
    }
}