use os_pipe;
use std::io::{Read, Write};
use std::thread;
use crate::task::*;
use crate::core::*;

pub struct ThreadTask<T: Send + 'static> {
    handle: Option<thread::JoinHandle<T>>,
    stdin: Box<dyn Write>,
    stdout: Box<dyn Read>,
    stderr: Box<dyn Read> ,
    
    result: Option<TaskResult<T>>
}

impl<T: Send + 'static> ThreadTask<T> {
    pub fn spawn<F>(f: F) -> ThreadTask<T>
    where
        F: FnOnce(Box<dyn Read>, Box<dyn Write>, Box<dyn Write>) -> T, //stdin reader, stdout writer adn stderr writer
        F: Send + 'static {

        let  pipes = (
            os_pipe::pipe().expect(sformat_error("Failed creating pipe for thread stdin").as_str()),
            os_pipe::pipe().expect(sformat_error("Failed creating pipe for thread stdout").as_str()),
            os_pipe::pipe().expect(sformat_error("Failed creating pipe for thread stderr").as_str()),
        );

        Self {
            stdin: Box::new(pipes.0.1),
            stdout: Box::new(pipes.1.0),
            stderr: Box::new(pipes.2.0),
            handle: Some(thread::spawn(move || {
                f(
                    Box::new(pipes.0.0),
                    Box::new(pipes.1.1),
                    Box::new(pipes.2.1),
                )
            })),
            
            result: None
        }
    }
}

impl<T: Send + 'static> Task<T> for ThreadTask<T> {
    fn stdin(&mut self) -> Option<&mut dyn Write> {
        Some(self.stdin.as_mut())
    }
    
    fn stdout(&mut self) -> Option<&mut dyn Read> {
        Some(self.stdout.as_mut())
    }

    fn stderr(&mut self) -> Option<&mut dyn Read> {
        Some(self.stderr.as_mut())
    }

    fn wait(&mut self) -> TaskResult<T> {

        let handle = self.handle.take().expect(sformat_error("Failed waiting for thread task as it does not exist").as_str());

        match handle.join() {
            Ok(v) => Ok(v),
            Err(_) => Err(TaskError::ThreadFailure("thread failed".to_string()))
        }
    }

    fn is_finished(&mut self) -> bool {
        match self.handle {
            Some(ref mut handle) => handle.is_finished(),
            None => true
        }
    }
}