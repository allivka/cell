use os_pipe;
use std::io::{Read, Write};
use std::thread;
use crate::task::*;
use crate::core::*;

// Owned by thread, reads from stdin and writes to stderr, stdout
pub struct ThreadPipeOwner {
    pub stdin: Option<Box<dyn Read>>,
    pub stdout: Option<Box<dyn Write>>,
    pub stderr: Option<Box<dyn Write>>,
}

// Owned from outside the thread and connects to it, writes to stdin and reads stdout, stderr
pub struct ThreadPipeConnector {
    pub stdin: Option<Box<dyn Write>>,
    pub stdout: Option<Box<dyn Read>>,
    pub stderr: Option<Box<dyn Read>>,
}


pub struct ThreadTask<T: Send + 'static> {
    handle: Option<thread::JoinHandle<T>>,
    connector: ThreadPipeConnector

}

impl<T: Send + 'static> ThreadTask<T> {
    pub fn spawn<F>(f: F) -> TaskResult<ThreadTask<T>>
    where
        F: FnOnce(ThreadPipeOwner) -> T, //stdin reader, stdout writer adn stderr writer
        F: Send + 'static {

        let  pipes = (
            os_pipe::pipe().expect(sformat_error("Failed creating pipe for thread stdin").as_str()),
            os_pipe::pipe().expect(sformat_error("Failed creating pipe for thread stdout").as_str()),
            os_pipe::pipe().expect(sformat_error("Failed creating pipe for thread stderr").as_str()),
        );

        Self {
            connector: ThreadPipeConnector {
                stdin: Some(Box::new(pipes.0.1)),
                stdout: Some(Box::new(pipes.1.0)),
                stderr: Some(Box::new(pipes.2.0)),
            },
            handle: Some(thread::spawn(move || {
                f(
                    ThreadPipeOwner {
                        stdin: Some(Box::new(pipes.0.0)),
                        stdout: Some(Box::new(pipes.1.1)),
                        stderr: Some(Box::new(pipes.2.1)),
                    }
                )
            })),

        }
    }
}

impl<T: Send + 'static> Task<T> for ThreadTask<T> {
    fn stdin(&mut self) -> Option<&mut dyn Write> {
        match self.connector.stdin {
            Some(ref mut stdin) => Some(stdin.as_mut()),
            None => None
        }
    }
    
    fn stdout(&mut self) -> Option<&mut dyn Read> {
        match self.connector.stdout {
            Some(ref mut stdout) => Some(stdout.as_mut()),
            None => None
        }
    }

    fn stderr(&mut self) -> Option<&mut dyn Read> {
        match self.connector.stderr {
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