use std::io::Write;
use crate::executor::*;
use crate::executor::ExecutorError::{NotFound, IoFailure};
use crate::task::TaskPipeOwner;

pub struct PWD {}
impl Executor for PWD {
    fn name(&self) -> &str {
        "pwd"
    }

    fn execute(&self, _args: ExecutorArgs, mut pipes: TaskPipeOwner) -> ExecutorResult<String> {
        let stdout = match pipes.stdout_writer {
            Some(ref mut writer) => writer,
            None => return Err(IoFailure(String::from("no stdout pipe provided")))
        };

        let s = match std::env::current_dir() {
            Ok(dir) => match dir.to_str() {
                Some(dir) => dir.to_string(),
                None => return Err(NotFound(String::from("Error: current working directory not found")))
            }
            Err(e) => return Err(IoFailure(format!("{} error; could not read current working directory: {}", self.name(), e)))
        };

        match stdout.write_fmt(format_args!("{}", s)) {
            Ok(_) => (),
            Err(e) => return Err(IoFailure(format!("{} error; could not write to stdout: {}", self.name(), e)))
        }

        Ok(String::new())
    }

    fn dyn_clone(&self) -> Box<dyn Executor> {
        Box::new(Self{})
    }
}